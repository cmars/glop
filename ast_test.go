package glop

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type marker struct {
	hit bool
}

func (m *marker) Do(_ *Context) {
	m.hit = true
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
	assert.True(t, m.hit)
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
	assert.True(t, fooMarker.hit)
	assert.True(t, barMarker.hit)
}
