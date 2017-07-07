package parser

import (
	"testing"

	"github.com/stretchr/testify/assert"

	"github.com/cmars/glop/ast"
	"github.com/cmars/glop/lexer"
)

func TestStartGotoStart(t *testing.T) {
	assert := assert.New(t)
	input := "state start { goto start }"

	l := lexer.New(input)
	p := New(l)

	root, err := p.Parse()
	assert.NotNil(root)
	assert.Nil(err)
	assert.Len(root.States, 1, "expected one state, found %+v", root.States)
	start, ok := root.States[0].(*ast.SingularState)
	assert.True(ok, "expected SingularState, got %+T", root.States[0])
	assert.NotNil(start.ID)
	assert.Equal("start", start.ID.Name, "expected 'start', got %q", start.ID.Name)
	assert.Len(start.Do, 1, "expected 1 action, found %d", len(start.Do))
	assert.Empty(start.Fault, "expected 0 fault actions, found %d", len(start.Fault))
}

func TestStartGotoStartFault(t *testing.T) {
	assert := assert.New(t)
	input := `state start { goto start } fault { log "bad things" }`

	l := lexer.New(input)
	p := New(l)

	root, err := p.Parse()
	assert.NotNil(root)
	assert.Nil(err)
	assert.Len(root.States, 1, "expected one state, found %+v", root.States)
	start, ok := root.States[0].(*ast.SingularState)
	assert.True(ok, "expected SingularState, got %+T", root.States[0])
	assert.Len(start.Fault, 1, "expected 1 fault action, found %d", len(start.Fault))
	log, ok := start.Fault[0].(*ast.LogAction)
	assert.True(ok, "expected LogAction, got %+T", start.Fault[0])
	assert.Equal("bad things", log.Message)
}

func TestEmptyInput(t *testing.T) {
	assert := assert.New(t)
	l := lexer.New("")
	p := New(l)

	root, err := p.Parse()
	assert.NotNil(root)
	assert.Nil(err)
	assert.Empty(root.States)
}

func TestBadInput(t *testing.T) {
	assert := assert.New(t)
	l := lexer.New("}")
	p := New(l)

	root, err := p.Parse()
	assert.Nil(root)
	assert.NotNil(err)
}

func TestSplit(t *testing.T) {
	assert := assert.New(t)
	input := "split start { foo | bar }"

	l := lexer.New(input)
	p := New(l)

	root, err := p.Parse()
	assert.NotNil(root)
	assert.Nil(err)
	assert.Len(root.States, 1, "expected one state, found %+v", root.States)
	start, ok := root.States[0].(*ast.SplitState)
	assert.True(ok, "expected SplitState, got %+T", root.States[0])
	assert.NotNil(start.ID)
	assert.Equal("start", start.ID.Name, "expected 'start', got %q", start.ID.Name)
	assert.Len(start.Splits, 2, "expected 2 split states, found %d", len(start.Splits))
	assert.Empty(start.Fault, "expected 0 fault actions, found %d", len(start.Fault))
}

func TestSplitFault(t *testing.T) {
	assert := assert.New(t)
	input := `split start { foo | bar } fault { log "bad things" }`

	l := lexer.New(input)
	p := New(l)

	root, err := p.Parse()
	assert.NotNil(root)
	assert.Nil(err)
	assert.Len(root.States, 1, "expected one state, found %+v", root.States)
	start, ok := root.States[0].(*ast.SplitState)
	assert.True(ok, "expected SplitState, got %+T", root.States[0])
	assert.NotNil(start.ID)
	assert.Len(start.Fault, 1, "expected 1 fault action, found %d", len(start.Fault))
	log, ok := start.Fault[0].(*ast.LogAction)
	assert.True(ok, "expected LogAction, got %+T", start.Fault[0])
	assert.Equal("bad things", log.Message)
}

func TestStates(t *testing.T) {
	assert := assert.New(t)
	input := `
state start {
  goto foo
}

states demo {
  state foo {
    log "in foo"
    goto bar
  }
  state bar {
    log "in bar"
    log "end"
  }
}`

	l := lexer.New(input)
	p := New(l)

	root, err := p.Parse()
	assert.NotNil(root)
	assert.Nil(err)
	assert.Len(root.States, 2)
	demo, ok := root.States[1].(*ast.NestedState)
	assert.True(ok, "expected NestedState, got %+T", root.States[1])
	assert.NotNil(demo.ID)
	assert.Equal("demo", demo.ID.Name, "expected 'demo', got %q", demo.ID.Name)
	assert.Len(demo.States, 2, "expected 2 nested states, found %d", len(demo.States))
	assert.Empty(demo.Fault, "expected 0 fault actions, found %d", len(demo.Fault))
}
