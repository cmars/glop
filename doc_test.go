package glop

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

var goodStateHelloWorld = []byte(`
states:
  start:
    do:
      - log: hello world
      - goto: start
    fault:
      - restart: {}
`)

func TestGoodStateHelloWorld(t *testing.T) {
	st, err := ParseYAML(goodStateHelloWorld)
	assert.NoError(t, err)
	require.NotNil(t, st)

	logDoc := LogDoc("hello world")
	gotoDoc := GotoDoc("start")
	assert.Equal(t, &StateDoc{
		States: map[string]*StateDoc{
			"start": &StateDoc{
				Do: []*ActionDoc{{
					Log: &logDoc,
				}, {
					Goto: &gotoDoc,
				}},
				Fault: []*ActionDoc{{
					Restart: &RestartDoc{},
				}}}}}, st)
}

var badStateAmbiguous = []byte(`
states:
  start:
    do:
      - log: hello world
      - goto: start
    states:
      more:
        do:
          - log: things
`)

func TestBadStateAmbiguous(t *testing.T) {
	_, err := ParseYAML(badStateAmbiguous)
	assert.NotNil(t, err)
	assert.True(t, IsNotValid(err))
	assert.Contains(t, err.Error(), "cannot declare both sub-states and actions")
}

func TestEmptyDoc(t *testing.T) {
	_, err := ParseYAML([]byte{})
	assert.NotNil(t, err)
	assert.True(t, IsNotValid(err))
	assert.Contains(t, err.Error(), "neither sub-states nor actions declared")
}

var goodWhen = []byte(`
do:
  - when:
      - condition: terrible
        do:
          - log: yuck
      - condition: lovely
        do:
          - log: yum
      - otherwise:
          - log: meh
`)

func TestGoodWhen(t *testing.T) {
	st, err := ParseYAML(goodWhen)
	assert.NoError(t, err)
	require.NotNil(t, st)
}

var badWhenOtherwises = []byte(`
do:
  - when:
      - otherwise:
          - log: this
      - condition: ok
        do:
          - log: that
`)

func TestBadWhenOtherwises(t *testing.T) {
	_, err := ParseYAML(badWhenOtherwises)
	assert.NotNil(t, err)
	assert.True(t, IsNotValid(err))
	assert.Contains(t, err.Error(), "otherwise must come last")
}

var goodWhenOneOtherwise = []byte(`
do:
  - when:
      # Useless but ok
      - otherwise:
          - log: this
`)

func TestGoodWhenOneOtherwise(t *testing.T) {
	st, err := ParseYAML(goodWhenOneOtherwise)
	assert.NoError(t, err)
	require.NotNil(t, st)
}

var badStateNoStart = []byte(`
states:
  foo:
    do:
      - log: foo
      - goto: bar
  bar:
    do:
      - log: bar
      - goto: foo
`)

func TestBadStateNoState(t *testing.T) {
	_, err := ParseYAML(badStateNoStart)
	assert.NotNil(t, err)
	assert.True(t, IsNotValid(err))
	assert.Contains(t, err.Error(), "missing start state")
}
