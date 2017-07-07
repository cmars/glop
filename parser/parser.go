package parser

import (
	"github.com/pkg/errors"

	"github.com/cmars/glop/ast"
	"github.com/cmars/glop/lexer"
	"github.com/cmars/glop/token"
)

type Parser struct {
	l *lexer.Lexer

	curToken  token.Token
	peekToken token.Token
}

func New(l *lexer.Lexer) *Parser {
	p := &Parser{l: l}

	// Read two tokens so both current and peek tokens are set.
	p.nextToken()
	p.nextToken()

	return p
}

func (p *Parser) nextToken() {
	p.curToken = p.peekToken
	p.peekToken = p.l.NextToken()
}

func (p *Parser) Parse() (*ast.NestedState, error) {
	root := &ast.NestedState{
		Token: token.Keyword(token.STATES),
		ID:    &ast.StateID{Token: token.Ident("(root)")},
	}
	for p.curToken.Type != token.EOF {
		st, err := p.parseState()
		if err != nil {
			return nil, errors.WithStack(err)
		}
		root.States = append(root.States, st)
		p.nextToken()
	}
	return root, nil
}

func (p *Parser) assertPeek(t token.TokenType) error {
	if p.peekToken.Type == t {
		p.nextToken()
		return nil
	}
	return errors.WithStack(p.peekError(t))
}

func (p *Parser) assert(t token.TokenType) error {
	if p.curToken.Type == t {
		p.nextToken()
		return nil
	}
	return errors.WithStack(p.expectError(t, p.curToken.Type))
}

func (p *Parser) peekError(t token.TokenType) error {
	return p.expectError(t, p.peekToken.Type)
}

func (p *Parser) expectError(expected, actual token.TokenType) error {
	return errors.Errorf("expected %q, got %q instead", expected, actual)
}

func (p *Parser) curError() error {
	return errors.Errorf("unexpected %q", p.curToken.Type)
}

func (p *Parser) parseState() (ast.State, error) {
	switch p.curToken.Type {
	case token.STATES:
		return p.parseNestedState()
	case token.STATE:
		return p.parseSingularState()
	case token.SPLIT:
		return p.parseSplitState()
	default:
		return nil, errors.WithStack(p.curError())
	}
}

func (p *Parser) parseNestedState() (*ast.NestedState, error) {
	panic("TODO")
}

func (p *Parser) parseSingularState() (*ast.SingularState, error) {
	st := &ast.SingularState{Token: p.curToken}
	if err := p.assertPeek(token.IDENT); err != nil {
		return nil, errors.WithStack(err)
	}
	st.ID = &ast.StateID{Token: p.curToken, Name: p.curToken.Literal}

	if err := p.assertPeek(token.LBRACE); err != nil {
		return nil, errors.WithStack(err)
	}
	p.nextToken() // LBRACE

	for p.curToken.Type != token.RBRACE {
		action, err := p.parseAction()
		if err != nil {
			return nil, errors.WithStack(err)
		}
		st.Do = append(st.Do, action)
		p.nextToken()
	}
	if p.peekToken.Type == token.FAULT {
		err := p.assert(token.RBRACE)
		if err != nil {
			return nil, errors.WithStack(err)
		}
		st.Fault, err = p.parseFault()
		if err != nil {
			return nil, errors.WithStack(err)
		}
	}
	return st, nil
}

func (p *Parser) parseFault() ([]ast.Action, error) {
	var actions []ast.Action
	if err := p.assert(token.FAULT); err != nil {
		return nil, errors.WithStack(err)
	}
	if err := p.assert(token.LBRACE); err != nil {
		return nil, errors.WithStack(err)
	}
	for p.curToken.Type != token.RBRACE {
		action, err := p.parseAction()
		if err != nil {
			return nil, errors.WithStack(err)
		}
		actions = append(actions, action)
		p.nextToken()
	}
	return actions, nil
}

func (p *Parser) parseSplitState() (*ast.SplitState, error) {
	st := &ast.SplitState{Token: p.curToken}
	if err := p.assertPeek(token.IDENT); err != nil {
		return nil, errors.WithStack(err)
	}
	st.ID = &ast.StateID{Token: p.curToken, Name: p.curToken.Literal}

	if err := p.assertPeek(token.LBRACE); err != nil {
		return nil, errors.WithStack(err)
	}

	for i := 0; p.curToken.Type != token.RBRACE; i++ {
		if i > 0 {
			if p.curToken.Type != token.PIPE {
				return nil, errors.WithStack(p.curError())
			}
		}

		if err := p.assertPeek(token.IDENT); err != nil {
			return nil, errors.WithStack(err)
		}
		splitState := &ast.StateID{Token: p.curToken, Name: p.curToken.Literal}
		st.Splits = append(st.Splits, splitState)
		p.nextToken()
	}
	if p.peekToken.Type == token.FAULT {
		err := p.assert(token.RBRACE)
		if err != nil {
			return nil, errors.WithStack(err)
		}
		st.Fault, err = p.parseFault()
		if err != nil {
			return nil, errors.WithStack(err)
		}
	}
	return st, nil
}

func (p *Parser) parseAction() (ast.Action, error) {
	switch p.curToken.Type {
	case token.GOTO:
		return p.parseGoto()
	case token.LOG:
		return p.parseLog()
	default:
		return nil, errors.WithStack(p.curError())
	}
}

func (p *Parser) parseGoto() (*ast.GotoAction, error) {
	g := &ast.GotoAction{Token: p.curToken}
	if err := p.assertPeek(token.IDENT); err != nil {
		return nil, errors.WithStack(err)
	}
	g.ID = &ast.StateID{Token: p.curToken, Name: p.curToken.Literal}
	return g, nil
}

func (p *Parser) parseLog() (*ast.LogAction, error) {
	g := &ast.LogAction{Token: p.curToken}
	if err := p.assertPeek(token.STRING); err != nil {
		return nil, errors.WithStack(err)
	}
	g.Message = p.curToken.Literal
	return g, nil
}
