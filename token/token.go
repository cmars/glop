package token

type TokenType string

type Token struct {
	Type    TokenType
	Literal string
}

const (
	ILLEGAL = "ILLEGAL"
	EOF     = "EOF"

	// Identifiers and literals
	IDENT    = "IDENT"
	INT      = "INT"
	STRING   = "STRING"
	DURATION = "DURATION"

	// Operators
	ASSIGN = "="
	PLUS   = "+"
	MINUS  = "-"
	EQUAL  = "=="
	NEQUAL = "!="
	GT     = ">"
	LT     = "<"
	GTE    = ">="
	LTE    = "<="

	// Delimiters
	COMMA  = ","
	LPAREN = "("
	RPAREN = ")"
	LBRACE = "{"
	RBRACE = "}"
	PIPE   = "|"

	// Keywords
	STATE     = "STATE"
	FAULT     = "FAULT"
	STATES    = "STATES"
	SPLIT     = "SPLIT"
	SPAWN     = "SPAWN"
	GOTO      = "GOTO"
	AWAIT     = "AWAIT"
	MESSAGE   = "MESSAGE"
	ELAPSED   = "ELAPSED"
	ASSERT    = "ASSERT"
	SEND      = "SEND"
	WHEN      = "WHEN"
	OTHERWISE = "OTHERWISE"
	LOG       = "LOG"
	SET       = "SET"
	TRUE      = "TRUE"
	FALSE     = "FALSE"
)

func Keyword(token string) Token {
	tt := TokenType(token)
	for k, v := range keywords {
		if v == tt {
			return Token{tt, k}
		}
	}
	return Token{tt, token}
}

func Ident(id string) Token {
	return Token{TokenType(IDENT), id}
}

var keywords = map[string]TokenType{
	"state":     STATE,
	"fault":     FAULT,
	"states":    STATES,
	"split":     SPLIT,
	"spawn":     SPAWN,
	"goto":      GOTO,
	"await":     AWAIT,
	"message":   MESSAGE,
	"elapsed":   ELAPSED,
	"assert":    ASSERT,
	"send":      SEND,
	"when":      WHEN,
	"otherwise": OTHERWISE,
	"log":       LOG,
	"set":       SET,
	"true":      TRUE,
	"false":     FALSE,
}

func LookupIdent(ident string) TokenType {
	if tok, ok := keywords[ident]; ok {
		return tok
	}
	return IDENT
}
