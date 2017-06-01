package glop

import (
	"testing"

	"github.com/pkg/errors"
	"github.com/stretchr/testify/assert"
)

type marker struct {
	hit int
}

func (m *marker) Do(_ *Context) {
	m.hit++
}

func TestSingular(t *testing.T) {
	m := &marker{}
	st := &SingularState{
		name:   "test",
		parent: nil,
		do:     []Action{m},
		fault:  nil,
	}
	c := NewContext(st)
	err := c.Run()
	assert.NoError(t, err)
	assert.Equal(t, m.hit, 1)
}

func TestCompositeGoto(t *testing.T) {
	fooMarker := &marker{}
	barMarker := &marker{}
	root := &CompositeState{
		name:   "test",
		states: map[string]State{},
	}
	root.AddSingular("start", []Action{Goto("foo")}, nil)
	root.AddSingular("foo", []Action{fooMarker, Goto("bar")}, nil)
	root.AddSingular("bar", []Action{barMarker, Goto("baz")}, nil)
	root.AddSingular("baz", nil, nil)
	c := NewContext(root)
	err := c.Run()
	assert.NoError(t, err)
	assert.Equal(t, fooMarker.hit, 1)
	assert.Equal(t, barMarker.hit, 1)
}

type faulter struct {
	err error
}

func (f *faulter) Do(c *Context) {
	c.Fault(f.err)
}

func TestSingularFault(t *testing.T) {
	unreachedMarker := &marker{}
	faultMarker := &marker{}
	st := &SingularState{
		name:   "test",
		parent: nil,
		do:     []Action{&faulter{err: errors.New("test fault")}, unreachedMarker},
		fault:  []Action{faultMarker},
	}
	c := NewContext(st)
	err := c.Run()
	assert.NotNil(t, err)
	assert.Equal(t, err.Error(), "test fault")
	assert.Equal(t, faultMarker.hit, 1)
	assert.Equal(t, unreachedMarker.hit, 0)
}

func TestNestedFault(t *testing.T) {
	unreachedMarker := &marker{}
	faultMarker := &marker{}
	st := &CompositeState{
		name:   "test",
		states: map[string]State{},
		fault:  []Action{faultMarker},
	}
	st.AddSingular("start", []Action{&faulter{err: errors.New("test fault")}, unreachedMarker}, nil)
	c := NewContext(st)
	err := c.Run()
	assert.NotNil(t, err)
	assert.Equal(t, err.Error(), "test fault")
	assert.Equal(t, faultMarker.hit, 1)
	assert.Equal(t, unreachedMarker.hit, 0)
}

func TestDoubleFaultNested(t *testing.T) {
	unreachedMarker := &marker{}
	faultMarker1 := &marker{}
	faultMarker2 := &marker{}
	st := &CompositeState{
		name:   "test",
		states: map[string]State{},
		fault:  nil,
	}
	st.AddComposite("start", nil, []Action{faultMarker2}).AddSingular(
		"start",
		[]Action{&faulter{err: errors.New("test fault")}, unreachedMarker},
		[]Action{faultMarker1, &faulter{err: errors.New("double fault")}})

	c := NewContext(st)
	err := c.Run()
	assert.NotNil(t, err)
	assert.Equal(t, err.Error(), "double fault")
	assert.Equal(t, faultMarker1.hit, 1)
	assert.Equal(t, faultMarker2.hit, 1)
	assert.Equal(t, unreachedMarker.hit, 0)
}

func TestUnhandledDoubleFaultNested(t *testing.T) {
	unreachedMarker := &marker{}
	faultMarker1 := &marker{}
	st := &CompositeState{
		name:   "test",
		states: map[string]State{},
		fault:  nil,
	}
	st.AddComposite("start", nil, nil).AddSingular(
		"start",
		[]Action{&faulter{err: errors.New("test fault")}, unreachedMarker},
		[]Action{faultMarker1, &faulter{err: errors.New("double fault")}})

	c := NewContext(st)
	err := c.Run()
	assert.NotNil(t, err)
	assert.Equal(t, err.Error(), "double fault")
	assert.Equal(t, faultMarker1.hit, 1)
	assert.Equal(t, unreachedMarker.hit, 0)
}

func TestWhenFoo(t *testing.T) {
	marker0 := &marker{}
	marker1 := &marker{}
	marker2 := &marker{}
	st := &CompositeState{
		name:   "test",
		states: map[string]State{},
		fault:  nil,
	}
	st.AddSingular(
		"start",
		[]Action{marker0,
			SetLocal{"foo": "1"},
			&When{
				Conditions: []Condition{{
					Expression: "foo == 1",
					Do:         []Action{marker1},
				}}}, marker2},
		nil,
	)

	c := NewContext(st)
	err := c.Run()
	assert.NoError(t, err)
	assert.Equal(t, marker0.hit, 1)
	assert.Equal(t, marker1.hit, 1)
	assert.Equal(t, marker2.hit, 1)
}

func TestWhenOtherwiseFoo(t *testing.T) {
	marker0 := &marker{}
	marker1 := &marker{}
	marker2 := &marker{}
	marker3 := &marker{}
	st := &CompositeState{
		name:   "test",
		states: map[string]State{},
		fault:  nil,
	}
	st.AddSingular(
		"start",
		[]Action{marker0,
			SetLocal{"foo": "1"},
			&When{
				Conditions: []Condition{{
					Expression: "foo != 1",
					Do:         []Action{marker1},
				}},
				Otherwise: []Action{marker2}}, marker3},
		nil,
	)

	c := NewContext(st)
	err := c.Run()
	assert.NoError(t, err)
	assert.Equal(t, marker0.hit, 1)
	assert.Equal(t, marker1.hit, 0)
	assert.Equal(t, marker2.hit, 1)
	assert.Equal(t, marker3.hit, 1)
}

func TestWhenBar(t *testing.T) {
	marker0 := &marker{}
	marker1 := &marker{}
	marker2 := &marker{}
	marker3 := &marker{}
	marker4 := &marker{}
	st := &CompositeState{
		name:   "test",
		states: map[string]State{},
		fault:  nil,
	}
	st.AddSingular(
		"start",
		[]Action{marker0,
			SetLocal{"foo": "2", "bar": "true"},
			&When{
				Conditions: []Condition{{
					Expression: "foo == 1",
					Do:         []Action{marker1},
				}, {
					Expression: "bar",
					Do:         []Action{marker2},
				}},
				Otherwise: []Action{marker3}}, marker4},
		nil,
	)

	c := NewContext(st)
	err := c.Run()
	assert.NoError(t, err)
	assert.Equal(t, marker0.hit, 1)
	assert.Equal(t, marker1.hit, 0)
	assert.Equal(t, marker2.hit, 1)
	assert.Equal(t, marker3.hit, 0)
	assert.Equal(t, marker4.hit, 1)
}
