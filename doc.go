package glop

import (
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/ghodss/yaml"
	"github.com/pkg/errors"
)

type Element interface {
	Validate() error
	Elements() []Element
}

func Visit(doc Element, f func(e Element) error) error {
	elements := []Element{doc}
	for len(elements) > 0 {
		e := elements[0]
		elements = elements[1:]
		if err := f(e); err != nil {
			return err
		}
		elements = append(e.Elements(), elements...)
	}
	return nil
}

func Validate(doc Element) error {
	return Visit(doc, func(e Element) error {
		return e.Validate()
	})
}

func NewNotValid(subject string) error {
	return &notValidError{subject: subject}
}

type notValidError struct {
	subject string
}

func (e *notValidError) Error() string {
	return e.subject + " not valid"
}

func IsNotValid(err error) bool {
	switch errors.Cause(err).(type) {
	case *notValidError:
		return true
	default:
		return false
	}
}

func ParseYAML(b []byte) (*StateDoc, error) {
	j, err := yaml.YAMLToJSON(b)
	if err != nil {
		return nil, errors.WithStack(err)
	}
	return ParseJSON(j)
}

func ParseJSON(b []byte) (*StateDoc, error) {
	var st StateDoc
	err := json.Unmarshal(b, &st)
	if err != nil {
		return nil, errors.WithStack(err)
	}
	return &st, Validate(&st)
}

type StateDoc struct {
	States map[string]*StateDoc `json:"states,omitempty"`
	Do     []*ActionDoc         `json:"do,omitempty"`
	Fault  []*ActionDoc         `json:"fault,omitempty"`
}

func (d *StateDoc) ToAST() (State, error) {
	if err := d.Validate(); err != nil {
		return nil, errors.WithStack(err)
	}
	if len(d.States) == 0 {
		result := &SingularState{}
		for _, actionDoc := range d.Do {
			action, err := actionDoc.ToAST()
			if err != nil {
				return nil, errors.WithStack(err)
			}
			result.do = append(result.do, action)
		}
		for _, actionDoc := range d.Fault {
			action, err := actionDoc.ToAST()
			if err != nil {
				return nil, errors.WithStack(err)
			}
			result.fault = append(result.fault, action)
		}
		return result, nil
	}

	result := &CompositeState{
		states: map[string]State{},
	}
	for k, v := range d.States {
		state, err := v.ToAST()
		if err != nil {
			return nil, errors.WithStack(err)
		}
		switch st := state.(type) {
		case *SingularState:
			st.name = k
			st.parent = result
		case *CompositeState:
			st.name = k
			st.parent = result
		default:
			panic(fmt.Sprintf("unknown state type %+T", state))
		}
		result.states[k] = state
	}
	return result, nil
}

func (d *StateDoc) Validate() error {
	if len(d.States) > 0 && len(d.Do) > 0 {
		return errors.Wrap(NewNotValid("state"), "cannot declare both sub-states and actions")
	}
	if len(d.States) == 0 && len(d.Do) == 0 {
		return errors.Wrap(NewNotValid("state"), "neither sub-states nor actions declared")
	}
	if len(d.States) > 0 {
		if _, ok := d.States["start"]; !ok {
			return errors.Wrap(NewNotValid("state"), "missing start state")
		}
	}
	return nil
}

func (d *StateDoc) Elements() []Element {
	var result []Element
	if len(d.States) > 0 {
		for _, e := range d.States {
			result = append(result, e)
		}
	} else if len(d.Do) > 0 {
		for _, e := range d.Do {
			result = append(result, e)
		}
	} else {
		panic("invalid state doc")
	}
	for _, e := range d.Fault {
		result = append(result, e)
	}
	return result
}

type ActionDoc struct {
	Assert   *AssertDoc   `json:"assert,omitempty"`
	Await    *AwaitDoc    `json:"await,omitempty"`
	Goto     *GotoDoc     `json:"goto,omitempty"`
	Log      *LogDoc      `json:"log,omitempty"`
	Restart  *RestartDoc  `json:"restart,omitempty"`
	Send     *SendDoc     `json:"send,omitempty"`
	Split    *SplitDoc    `json:"split,omitempty"`
	When     *WhenDoc     `json:"when,omitempty"`
	Shutdown *ShutdownDoc `json:"shutdown,omitempty"`
}

func (d *ActionDoc) ToAST() (Action, error) {
	if err := d.Validate(); err != nil {
		return nil, errors.WithStack(err)
	}
	switch {
	case d.Assert != nil:
		return Assert(*d.Assert), errors.WithStack(d.Assert.Validate())
	case d.Await != nil:
		if err := d.Await.Validate(); err != nil {
			return nil, errors.WithStack(err)
		}
		var result Await
		for _, evDoc := range *d.Await {
			ev, err := evDoc.ToAST()
			if err != nil {
				return nil, errors.WithStack(err)
			}
			result = append(result, ev)
		}
		return result, nil
	case d.Goto != nil:
		return Goto(*d.Goto), errors.WithStack(d.Goto.Validate())
	case d.Log != nil:
		return Log(*d.Log), errors.WithStack(d.Log.Validate())
	case d.Restart != nil:
		panic("not implemented yet")
	case d.Send != nil:
		contents := map[string]string{}
		if d.Send.Contents != nil {
			for k, v := range d.Send.Contents {
				contents[k] = v
			}
		}
		return &Send{
			Topic:    d.Send.Topic,
			Contents: contents,
		}, errors.WithStack(d.Send.Validate())
	case d.Split != nil:
		if err := d.Split.Validate(); err != nil {
			return nil, errors.WithStack(err)
		}
		var result Split
		for _, epDoc := range *d.Split {
			ep, err := epDoc.ToAST()
			if err != nil {
				return nil, errors.WithStack(err)
			}
			result = append(result, ep)
		}
		return result, nil
	case d.When != nil:
		if err := d.When.Validate(); err != nil {
			return nil, errors.WithStack(err)
		}
		result := &When{}
		for _, condDoc := range *d.When {
			err := condDoc.Validate()
			if err != nil {
				return nil, errors.WithStack(err)
			}
			var actions []Action
			var actionDocs []ActionDoc
			if condDoc.Condition != "" {
				actionDocs = condDoc.Do
			} else {
				actionDocs = condDoc.Otherwise
			}
			for _, actionDoc := range actionDocs {
				action, err := actionDoc.ToAST()
				if err != nil {
					return nil, errors.WithStack(err)
				}
				actions = append(actions, action)
			}
			if condDoc.Condition != "" {
				result.Conditions = append(result.Conditions, Condition{
					Expression: condDoc.Condition,
					Do:         actions,
				})
			} else {
				result.Otherwise = actions
			}
		}
		return result, nil
	case d.Shutdown != nil:
		return &Shutdown{}, nil
	default:
		panic("invalid action")
	}
}

func (d *ActionDoc) Validate() error {
	var types []string
	if d.Assert != nil {
		types = append(types, "assert")
	}
	if d.Await != nil {
		types = append(types, "await")
	}
	if d.Goto != nil {
		types = append(types, "goto")
	}
	if d.Log != nil {
		types = append(types, "log")
	}
	if d.Restart != nil {
		types = append(types, "restart")
	}
	if d.Send != nil {
		types = append(types, "send")
	}
	if d.Split != nil {
		types = append(types, "split")
	}
	if d.When != nil {
		types = append(types, "when")
	}
	if d.Shutdown != nil {
		types = append(types, "shutdown")
	}
	if len(types) == 0 {
		return errors.Wrap(NewNotValid("action"), "missing or unknown type")
	}
	if len(types) > 1 {
		return errors.Wrapf(NewNotValid("action"), "multiple types declared: %s", strings.Join(types, " "))
	}
	return nil
}

func (d *ActionDoc) Elements() []Element {
	if d.Assert != nil {
		return []Element{d.Assert}
	}
	if d.Await != nil {
		return []Element{d.Await}
	}
	if d.Goto != nil {
		return []Element{d.Goto}
	}
	if d.Log != nil {
		return []Element{d.Log}
	}
	if d.Restart != nil {
		return []Element{d.Restart}
	}
	if d.Send != nil {
		return []Element{d.Send}
	}
	if d.Split != nil {
		return []Element{d.Split}
	}
	if d.When != nil {
		return []Element{d.When}
	}
	if d.Shutdown != nil {
		return []Element{d.Shutdown}
	}
	panic("invalid action")
}

type AssertDoc string

func (d *AssertDoc) Validate() error {
	if *d == "" {
		return errors.Wrap(NewNotValid("assert"), "missing assert condition")
	}
	return nil
}

func (d *AssertDoc) Elements() []Element { return nil }

type AwaitDoc []*EventDoc

func (d AwaitDoc) Validate() error {
	return nil
}

func (d AwaitDoc) Elements() []Element {
	var result []Element
	for _, e := range d {
		result = append(result, e)
	}
	return result
}

type EventDoc struct {
	Message *MessageEventDoc `json:"message,omitempty"`
	Elapsed *ElapsedEventDoc `json:"elapsed,omitempty"`
	Join    *JoinEventDoc    `json:"join,omitempty"`
}

func (d *EventDoc) ToAST() (EventHandler, error) {
	err := d.Validate()
	if err != nil {
		return nil, errors.WithStack(err)
	}
	switch {
	case d.Message != nil:
		if err := d.Message.Validate(); err != nil {
			return nil, errors.WithStack(err)
		}
		var actions []Action
		for _, actionDoc := range d.Message.Do {
			action, err := actionDoc.ToAST()
			if err != nil {
				return nil, errors.WithStack(err)
			}
			actions = append(actions, action)
		}
		return &MessageEvent{
			Topic:   d.Message.Topic,
			Actions: actions,
		}, nil
	case d.Elapsed != nil:
		if err := d.Elapsed.Validate(); err != nil {
			return nil, errors.WithStack(err)
		}
		duration, err := time.ParseDuration(d.Elapsed.Duration)
		if err != nil {
			panic(err)
		}
		var actions []Action
		for _, actionDoc := range d.Elapsed.Do {
			action, err := actionDoc.ToAST()
			if err != nil {
				return nil, errors.WithStack(err)
			}
			actions = append(actions, action)
		}
		return &ElapsedEvent{
			Duration: duration,
			Actions:  actions,
		}, nil
	case d.Join != nil:
		if err := d.Join.Validate(); err != nil {
			return nil, errors.WithStack(err)
		}
		var actions []Action
		for _, actionDoc := range d.Join.Do {
			action, err := actionDoc.ToAST()
			if err != nil {
				return nil, errors.WithStack(err)
			}
			actions = append(actions, action)
		}
		return &JoinEvent{
			Actions: actions,
		}, nil
	default:
		panic("invalid action")
	}
}

func (d *EventDoc) Validate() error {
	var types []string
	if d.Message != nil {
		types = append(types, "message")
	}
	if d.Elapsed != nil {
		types = append(types, "elapsed")
	}
	if d.Join != nil {
		types = append(types, "join")
	}
	if len(types) == 0 {
		return NewNotValid("event")
	}
	if len(types) > 1 {
		return errors.Wrapf(NewNotValid("event"), "multiple types declared: %s", strings.Join(types, " "))
	}
	return nil
}

func (d *EventDoc) Elements() []Element {
	if d.Message != nil {
		return []Element{d.Message}
	}
	if d.Elapsed != nil {
		return []Element{d.Elapsed}
	}
	if d.Join != nil {
		return []Element{d.Join}
	}
	panic("invalid event")
}

type MessageEventDoc struct {
	Role  string      `json:"role"`
	Topic string      `json:"topic"`
	Do    []ActionDoc `json:"do"`
}

func (d *MessageEventDoc) Validate() error {
	if d.Topic == "" {
		return errors.Wrap(NewNotValid("message"), "missing topic")
	}
	return nil
}

func (d *MessageEventDoc) Elements() []Element { return nil }

type ElapsedEventDoc struct {
	Duration string      `json:"duration"`
	Do       []ActionDoc `json:"do"`
}

func (d *ElapsedEventDoc) Validate() error {
	if _, err := time.ParseDuration(d.Duration); err != nil {
		return NewNotValid(err.Error())
	}
	return nil
}

func (d *ElapsedEventDoc) Elements() []Element { return nil }

type JoinEventDoc struct {
	Do []ActionDoc `json:"do"`
}

func (d *JoinEventDoc) Validate() error { return nil }

func (d *JoinEventDoc) Elements() []Element { return nil }

type GotoDoc string

func (d *GotoDoc) Validate() error {
	if *d == "" {
		return NewNotValid("goto")
	}
	return nil
}

func (d *GotoDoc) Elements() []Element { return nil }

type LogDoc string

func (d *LogDoc) Validate() error {
	if *d == "" {
		return NewNotValid("log")
	}
	return nil
}

func (d *LogDoc) Elements() []Element { return nil }

type RestartDoc struct{}

func (d *RestartDoc) Validate() error { return nil }

func (d *RestartDoc) Elements() []Element { return nil }

type ShutdownDoc struct{}

func (d *ShutdownDoc) Validate() error { return nil }

func (d *ShutdownDoc) Elements() []Element { return nil }

type SendDoc struct {
	Dst      string            `json:"dst"`
	Role     string            `json:"role"`
	Topic    string            `json:"topic"`
	Contents map[string]string `json:"contents"`
}

func (d *SendDoc) Validate() error {
	if d.Topic == "" {
		return errors.Wrap(NewNotValid("send"), "missing topic")
	}
	return nil
}

func (d *SendDoc) Elements() []Element { return nil }

type SplitDoc []*EntryPointDoc

func (d SplitDoc) Validate() error {
	if len(d) == 0 {
		return errors.Wrap(NewNotValid("split"), "missing entry points")
	}
	return nil
}

func (d SplitDoc) Elements() []Element {
	var result []Element
	for _, e := range d {
		result = append(result, e)
	}
	return result
}

type EntryPointDoc struct {
	Enter string            `json:"enter"`
	Set   map[string]string `json:"set,omitempty"`
}

func (d *EntryPointDoc) ToAST() (SplitEntry, error) {
	if err := d.Validate(); err != nil {
		return SplitEntry{}, errors.WithStack(err)
	}
	result := SplitEntry{
		State: d.Enter,
	}
	for k, v := range d.Set {
		result.Locals[k] = v
	}
	return result, nil
}

func (d *EntryPointDoc) Validate() error {
	if d.Enter == "" {
		return errors.Wrap(NewNotValid("enter"), "missing entry state")
	}
	return nil
}

func (d *EntryPointDoc) Elements() []Element { return nil }

type WhenDoc []*ConditionalDoc

func (d WhenDoc) Validate() error {
	if len(d) == 0 {
		return errors.Wrap(NewNotValid("when"), "missing conditions")
	}
	for i := 0; i < len(d); i++ {
		if len(d[i].Otherwise) > 0 {
			if i != len(d)-1 {
				return errors.Wrap(NewNotValid("when"), "otherwise must come last")
			}
		}
	}
	return nil
}

func (d WhenDoc) Elements() []Element {
	var result []Element
	for _, e := range d {
		result = append(result, e)
	}
	return result
}

type ConditionalDoc struct {
	Condition string      `json:"condition"`
	Do        []ActionDoc `json:"do"`
	Otherwise []ActionDoc `json:"otherwise"`
}

func (d *ConditionalDoc) Validate() error {
	if len(d.Do) > 0 {
		if d.Condition == "" {
			return errors.Wrap(NewNotValid("condition"), "missing condition")
		}
	} else if len(d.Otherwise) > 0 {
		if d.Condition != "" {
			return errors.Wrap(NewNotValid("otherwise"), "otherwise cannot have a condition")
		}
	}
	return nil
}

func (d *ConditionalDoc) Elements() []Element { return nil }
