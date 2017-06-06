package glop

import (
	"fmt"
	"log"
	"reflect"
	"sync"
	"time"

	"github.com/Knetic/govaluate"
	"github.com/pkg/errors"
	"gopkg.in/tomb.v2"
)

type State interface {
	Fault() []Action
	Name() string
	Parent() *CompositeState
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

func (s *SingularState) Parent() *CompositeState { return s.parent }

type CompositeState struct {
	name   string
	parent *CompositeState
	states map[string]State
	fault  []Action
}

func (s *CompositeState) AddSingular(name string, do []Action, fault []Action) *SingularState {
	st := &SingularState{
		name:   name,
		parent: s,
		do:     do,
		fault:  fault,
	}
	s.states[name] = st
	return st
}

func (s *CompositeState) AddComposite(name string, states map[string]State, fault []Action) *CompositeState {
	if states == nil {
		states = map[string]State{}
	}
	st := &CompositeState{
		name:   name,
		parent: s,
		states: states,
		fault:  fault,
	}
	s.states[name] = st
	return st
}

func (s *CompositeState) Fault() []Action { return s.fault }

func (s *CompositeState) Name() string { return s.name }

func (s *CompositeState) Parent() *CompositeState { return s.parent }

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
	splits  *tomb.Tomb
	bus     *MessageBus
}

func NewContext(state State) *Context {
	c := &Context{globals: map[string]interface{}{}, splits: &tomb.Tomb{}, bus: NewMessageBus()}
	c.Enter(state, nil)
	return c
}

func (c *Context) Spawn() {
	c.splits.Go(func() error {
		for c.Next() {
			a := c.Action()
			a.Do(c)
		}
		return c.fault
	})
}

func (c *Context) Run() error {
	defer c.bus.Close()
	c.Spawn()
	return c.splits.Wait()
}

func (c *Context) State(name string) State {
	if c.state.Name() == name {
		return c.state
	}
	cs := c.state.Parent()
	for cs != nil {
		st, ok := cs.states[name]
		if ok {
			return st
		}
		cs = cs.Parent()
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

func (c *Context) Enter(state State, locals map[string]interface{}) {
	if locals == nil {
		locals = map[string]interface{}{}
	}
	switch st := state.(type) {
	case *SingularState:
		c.state = st
		actions := make([]Action, len(st.do))
		for i := range st.do {
			actions[i] = st.do[i]
		}
		c.actions = actions
		c.index = -1
		c.locals = locals
	case *CompositeState:
		c.Enter(st.Start(), locals)
	default:
		panic(fmt.Sprintf("invalid state type: %+T", state))
	}
}

func (c *Context) Fault(err error) bool {
	st := c.state
	for st != nil {
		fault := st.Fault()
		if len(fault) == 0 || c.fault != nil {
			c.fault = nil
			if parent := st.Parent(); parent != nil {
				st = parent
			} else {
				// Set the interface value to nil -- not its target.
				st = nil
			}
			c.splits.Kill(nil)
			c.locals = map[string]interface{}{}
			continue
		}
		c.fault = err
		c.state = st
		c.actions = st.Fault()
		c.index = -1
		return true
	}
	c.fault = err
	return false
}

func (c *Context) Split(st State, locals map[string]interface{}) *Context {
	// Give the child context its own copy of the global variables.
	// Like a fork(2).
	globals := map[string]interface{}{}
	for k := range c.globals {
		globals[k] = c.globals[k]
	}
	childC := &Context{
		globals: globals,
		splits:  c.splits,
		bus:     c.bus,
	}
	childC.Enter(st, locals)
	return childC
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
		c.Enter(st, nil)
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
	if isTrue(result) {
		return
	}
	c.Fault(errors.New("assertion failed"))
}

func isTrue(v interface{}) bool {
	switch val := v.(type) {
	case bool:
		return val
	case float64:
		return val != 0.0
	case string:
		return val != ""
	case []interface{}:
		return len(val) > 0
	}
	return false
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

type When struct {
	Conditions []Condition
	Otherwise  []Action
}

type Condition struct {
	Expression string
	Do         []Action
}

func (a *When) Do(c *Context) {
	for _, cond := range a.Conditions {
		expr, err := govaluate.NewEvaluableExpression(cond.Expression)
		if err != nil {
			c.Fault(errors.Wrap(err, "when: invalid expression"))
			return
		}
		result, err := expr.Evaluate(c.Parameters())
		if err != nil {
			c.Fault(errors.WithMessage(err, "when: cannot evaluate condition"))
			return
		}
		if isTrue(result) {
			c.actions = append(append(c.actions[:c.index], cond.Do...), c.actions[c.index:]...)
			return
		}
	}
	if len(a.Otherwise) > 0 {
		c.actions = append(append(c.actions[:c.index], a.Otherwise...), c.actions[c.index:]...)
	}
}

var ErrShutdown = errors.New("shutdown")

type Shutdown struct{}

func (a *Shutdown) Do(c *Context) {
	c.actions = c.actions[:c.index+1]
	c.splits.Kill(ErrShutdown)
}

type Split []SplitEntry

type SplitEntry struct {
	State  string
	Locals map[string]string
}

func (a Split) Do(c *Context) {
	var err error
	var contexts []*Context
CONTEXTS:
	for _, e := range a {
		st := c.State(e.State)
		if st == nil {
			err = errors.Errorf("state not found: %q", a)
			break
		}
		locals := map[string]interface{}{}
		params := c.Parameters()
		for k, v := range e.Locals {
			var expr *govaluate.EvaluableExpression
			var result interface{}
			expr, err = govaluate.NewEvaluableExpression(v)
			if err != nil {
				break CONTEXTS
			}
			result, err = expr.Evaluate(params)
			if err != nil {
				break CONTEXTS
			}
			locals[k] = result
		}
		contexts = append(contexts, c.Split(st, locals))
	}
	if err != nil {
		c.Fault(err)
		return
	}
	for _, child := range contexts {
		child.Spawn()
	}
}

type Await []EventHandler

type EventHandler interface {
	Do() []Action
	SelectCase(c *Context) reflect.SelectCase
}

type ElapsedEvent struct {
	Duration time.Duration
	Actions  []Action
}

func (ev *ElapsedEvent) SelectCase(_ *Context) reflect.SelectCase {
	return reflect.SelectCase{
		Dir:  reflect.SelectRecv,
		Chan: reflect.ValueOf(time.NewTimer(ev.Duration).C),
	}
}

func (ev *ElapsedEvent) Do() []Action { return ev.Actions }

type JoinEvent struct {
	Actions []Action
}

func (ev *JoinEvent) SelectCase(c *Context) reflect.SelectCase {
	return reflect.SelectCase{
		Dir:  reflect.SelectRecv,
		Chan: reflect.ValueOf(c.splits.Dying()),
	}
}

func (ev *JoinEvent) Do() []Action { return ev.Actions }

type MessageEvent struct {
	Topic   string
	Actions []Action
}

func (ev *MessageEvent) SelectCase(c *Context) reflect.SelectCase {
	return reflect.SelectCase{
		Dir:  reflect.SelectRecv,
		Chan: reflect.ValueOf(c.bus.SubscribeTopic(ev.Topic)),
	}
}

func (ev *MessageEvent) Do() []Action { return ev.Actions }

func (a Await) Do(c *Context) {
	var cases []reflect.SelectCase
	for _, ev := range a {
		cases = append(cases, ev.SelectCase(c))
	}
	i, _, _ := reflect.Select(cases)
	c.actions = append(append(c.actions[:c.index], a[i].Do()...), c.actions[c.index:]...)
}

type MessageBus struct {
	t tomb.Tomb

	mu        sync.RWMutex
	topicsIn  map[string]chan *Message
	topicsOut map[string]chan *Message
}

type Message struct {
	Topic    string
	Contents map[string]interface{}
}

func NewMessageBus() *MessageBus {
	return &MessageBus{
		topicsIn:  map[string]chan *Message{},
		topicsOut: map[string]chan *Message{},
	}
}

func (mb *MessageBus) Publish(m *Message) {
	mb.publishTopic(m)
	// TODO: publishRole
}

func (mb *MessageBus) publishTopic(m *Message) {
	mb.mu.Lock()
	defer mb.mu.Unlock()

	chOut, ok := mb.topicsOut[m.Topic]
	if !ok {
		chOut = make(chan *Message)
		mb.topicsOut[m.Topic] = chOut
	}

	chIn, ok := mb.topicsIn[m.Topic]
	if !ok {
		chIn = make(chan *Message)
		mb.topicsIn[m.Topic] = chIn
		mb.t.Go(mb.relay(chIn, chOut))
	}
	select {
	case chIn <- m:
	case <-mb.t.Dying():
	}
}

func (mb *MessageBus) relay(chIn <-chan *Message, chOut chan<- *Message) func() error {
	return func() error {
		var q []*Message
		for {
			if len(q) > 0 {
				select {
				case chOut <- q[0]:
					q = q[1:]
				case m := <-chIn:
					q = append(q, m)
				case <-mb.t.Dying():
					return nil
				}
			} else {
				select {
				case m := <-chIn:
					q = append(q, m)
				case <-mb.t.Dying():
					return nil
				}
			}
		}
	}
}

func (mb *MessageBus) SubscribeTopic(topic string) <-chan *Message {
	mb.mu.RLock()
	ch, ok := mb.topicsOut[topic]
	mb.mu.RUnlock()
	if !ok {
		mb.mu.Lock()
		ch = make(chan *Message)
		mb.topicsOut[topic] = ch
		mb.mu.Unlock()
	}
	return ch
}

func (mb *MessageBus) Close() error {
	// Issue kill in a goroutine in case we've never spawned one. Otherwise the
	// tomb won't enter a dying state even if killed.
	mb.t.Go(func() error {
		mb.t.Kill(nil)
		return nil
	})
	return mb.t.Wait()
}

type Send struct {
	Topic    string
	Contents map[string]string
}

func (a *Send) Do(c *Context) {
	contents, err := a.eval(c)
	if err != nil {
		c.Fault(err)
		return
	}
	c.bus.Publish(&Message{Topic: a.Topic, Contents: contents})
}

func (a *Send) eval(c *Context) (map[string]interface{}, error) {
	contents := map[string]interface{}{}
	if a.Contents == nil {
		return contents, nil
	}
	params := c.Parameters()
	for k, v := range a.Contents {
		expr, err := govaluate.NewEvaluableExpression(v)
		if err != nil {
			return nil, errors.Wrapf(err, "send: %q: invalid expression", k)
		}
		result, err := expr.Evaluate(params)
		if err != nil {
			return nil, errors.Wrapf(err, "send: %q: cannot evaluate", k)
		}
		contents[k] = result
	}
	return contents, nil
}
