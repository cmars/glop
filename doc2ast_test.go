package glop

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

var helloWorld = []byte(`
states:
  start:
    do:
      - split:
          - enter: hello
          - enter: world
      - await:
          - elapsed:
              duration: 10s
              do:
                - send:
                    topic: shutdown
                - log: shutdown initiated
      - await:
          - join: {}
      - log: shutdown complete
      - shutdown: {}
  hello:
    do:
      - await:
          - elapsed:
              duration: 1s
              do:
                - log: hello
          - message:
              topic: shutdown
              do:
                - log: shutdown received
                - shutdown: {}
          - join:
              do:
                - shutdown: {}
      - goto: hello
  world:
    do:
      - await:
          - elapsed:
              duration: 1s
              do:
                - log: world
          - message:
              topic: shutdown
              do:
                - log: shutdown received
                - shutdown: {}
          - join:
              do:
                - shutdown: {}
      - goto: world
`)

func TestDoc2Ast(t *testing.T) {
	doc, err := ParseYAML(helloWorld)
	assert.NoError(t, err)
	st, err := doc.ToAST()
	assert.NoError(t, err)
	assert.NotNil(t, st)
	//c := NewContext(st)
	//log.Printf("%+v", st)
	//err = c.Run()
	//assert.Equal(t, ErrShutdown, err)
}
