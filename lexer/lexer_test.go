package lexer

import (
	"testing"

	"github.com/stretchr/testify/assert"

	"github.com/cmars/glop/token"
)

type expectedTokens struct {
	expectedType    token.TokenType
	expectedLiteral string
}

func expect(t *testing.T, input string, expected []expectedTokens) {
	assert := assert.New(t)
	l := New(input)
	for i, tt := range expected {
		tok := l.NextToken()
		assert.Equal(tok.Type, tt.expectedType,
			"tests[%d] - tokentype wrong. expected=%q, got=%q",
			i, tt.expectedType, tok.Type)
		assert.Equal(tok.Literal, tt.expectedLiteral,
			"tests[%d] - literal wrong. expected=%q, got=%q",
			i, tt.expectedLiteral, tok.Literal)
	}
}

func TestNextToken(t *testing.T) {
	input := `=+(){},`
	expect(t, input, []expectedTokens{
		{token.ASSIGN, "="},
		{token.PLUS, "+"},
		{token.LPAREN, "("},
		{token.RPAREN, ")"},
		{token.LBRACE, "{"},
		{token.RBRACE, "}"},
		{token.COMMA, ","},
		{token.EOF, ""},
	})
}

func TestNextTokenKeywords(t *testing.T) {
	input := "" +
		`state start {
	await {
		message (topic "stuff", role "reader") msg {
			log "got msg"
		}
		elapsed 15s {
			log "timeout"
		}
	}
	goto start
} fault {
	log "bad things"
}`
	expect(t, input, []expectedTokens{
		{token.STATE, "state"},
		{token.IDENT, "start"},
		{token.LBRACE, "{"},
		{token.AWAIT, "await"},
		{token.LBRACE, "{"},
		{token.MESSAGE, "message"},
		{token.LPAREN, "("},
		{token.IDENT, "topic"},
		{token.STRING, "stuff"},
		{token.COMMA, ","},
		{token.IDENT, "role"},
		{token.STRING, "reader"},
		{token.RPAREN, ")"},
		{token.IDENT, "msg"},
		{token.LBRACE, "{"},
		{token.LOG, "log"},
		{token.STRING, "got msg"},
		{token.RBRACE, "}"},
		{token.ELAPSED, "elapsed"},
		{token.DURATION, "15s"},
		{token.LBRACE, "{"},
		{token.LOG, "log"},
		{token.STRING, "timeout"},
		{token.RBRACE, "}"},
		{token.RBRACE, "}"},
		{token.GOTO, "goto"},
		{token.IDENT, "start"},
		{token.RBRACE, "}"},
		{token.FAULT, "fault"},
		{token.LBRACE, "{"},
		{token.LOG, "log"},
		{token.STRING, "bad things"},
		{token.RBRACE, "}"},
	})
}

func TestTokenizeSplit(t *testing.T) {
	input := "split start { pinger | ponger } fault { goto start }"
	expect(t, input, []expectedTokens{
		{token.SPLIT, "split"},
		{token.IDENT, "start"},
		{token.LBRACE, "{"},
		{token.IDENT, "pinger"},
		{token.PIPE, "|"},
		{token.IDENT, "ponger"},
		{token.RBRACE, "}"},
		{token.FAULT, "fault"},
		{token.LBRACE, "{"},
		{token.GOTO, "goto"},
		{token.IDENT, "start"},
		{token.RBRACE, "}"},
	})
}
