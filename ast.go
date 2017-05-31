package glop

import (
	"fmt"
	"log"

	"github.com/Knetic/govaluate"
	"github.com/pkg/errors"
)

type State interface {
	Fault() []Action
	Name() string
	Parent() State
}

type Action interface {
	Do(c *Context)
}

type SingularState struct {
	name   string
	parent *CompositeState
	do     []Action
	fault  []Action
}

func (s *SingularState) Fault() []Action { return s.fault }

func (s *SingularState) Name() string { return s.name }

func (s *SingularState) Parent() State { return s.parent }

type CompositeState struct {
	name   string
	parent *CompositeState
	states map[string]State
	fault  []Action
}

func (s *CompositeState) AddSingular(name string, do []Action, fault []Action) {
	s.states[name] = &SingularState{
		name:   name,
		parent: s,
		do:     do,
		fault:  fault,
	}
}

func (s *CompositeState) AddComposite(name string, states map[string]State, fault []Action) {
	s.states[name] = &CompositeState{
		name:   name,
		parent: s,
		states: states,
		fault:  fault,
	}
}

func (s *CompositeState) Fault() []Action { return s.fault }

func (s *CompositeState) Name() string { return s.name }

func (s *CompositeState) Parent() State { return s.parent }

func (s *CompositeState) Start() State {
	st, ok := s.states["start"]
	if !ok {
		panic("missing start state")
	}
	return st
}

type Context struct {
	state   State
	actions []Action
	index   int
	fault   error
	locals  map[string]interface{}
	globals map[string]interface{}
}

func NewContext(state State) *Context {
	c := &Context{globals: map[string]interface{}{}}
	c.Enter(state)
	return c
}

func (c *Context) Run() error {
	for c.Next() {
		a := c.Action()
		a.Do(c)
	}
	return c.fault
}

func (c *Context) State(name string) State {
	if c.state.Name() == name {
		return c.state
	}
	cs := c.state.Parent().(*CompositeState)
	for cs != nil {
		st, ok := cs.states[name]
		if ok {
			return st
		}
		cs = c.state.Parent().(*CompositeState)
	}
	return nil
}

func (c *Context) Parameters() map[string]interface{} {
	result := map[string]interface{}{}
	for k, v := range c.globals {
		result[k] = v
	}
	for k, v := range c.locals {
		result[k] = v
	}
	return result
}

func (c *Context) Enter(state State) {
	switch st := state.(type) {
	case *SingularState:
		c.state = st
		c.actions = st.do
		c.index = -1
		c.locals = map[string]interface{}{}
	case *CompositeState:
		c.Enter(st.Start())
	default:
		panic(fmt.Sprintf("invalid state type: %+T", state))
	}
}

func (c *Context) Fault(err error) bool {
	st := c.state
	for st != nil {
		fault := st.Fault()
		if len(fault) == 0 {
			st = st.Parent()
			c.locals = map[string]interface{}{}
			continue
		}
		c.fault = err
		c.state = st
		c.actions = st.Fault()
		c.index = -1
		return true
	}
	return false
}

func (c *Context) Next() bool {
	c.index++
	return c.index < len(c.actions)
}

func (c *Context) Action() Action {
	return c.actions[c.index]
}

type Goto string

func (a Goto) Do(c *Context) {
	st := c.State(string(a))
	if st == nil {
		c.Fault(errors.Errorf("state not found: %q", a))
	} else {
		c.Enter(st)
	}
}

type Assert string

func (a Assert) Do(c *Context) {
	expr, err := govaluate.NewEvaluableExpression(string(a))
	if err != nil {
		c.Fault(errors.WithMessage(err, "assert: invalid expression"))
		return
	}
	result, err := expr.Evaluate(c.Parameters())
	if err != nil {
		c.Fault(errors.WithMessage(err, "assert: cannot evaluate"))
		return
	}
	if b, ok := result.(bool); b && ok {
		return
	}
	c.Fault(errors.New("assertion failed"))
}

type SetLocal map[string]string

func (a SetLocal) Do(c *Context) {
	for k, v := range a {
		expr, err := govaluate.NewEvaluableExpression(string(v))
		if err != nil {
			c.Fault(errors.Wrapf(err, "set: %q: invalid expression", k))
			return
		}
		result, err := expr.Evaluate(c.Parameters())
		if err != nil {
			c.Fault(errors.Wrapf(err, "set: %q: cannot evaluate", k))
			return
		}
		c.locals[k] = result
	}
}

type Log string

func (a Log) Do(c *Context) {
	log.Println(a)
}
