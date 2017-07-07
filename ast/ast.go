package ast

import (
	"github.com/cmars/glop/token"
)

type Node interface {
	TokenLiteral() string
}

type State interface {
	Node
	stateNode()
}

type Action interface {
	Node
	actionNode()
}

type Expression interface {
	Node
	expressionNode()
}

type SingularState struct {
	Token token.Token
	ID    *StateID
	Do    []Action
	Fault []Action
}

func (*SingularState) stateNode()             {}
func (s *SingularState) TokenLiteral() string { return s.Token.Literal }

type NestedState struct {
	Token  token.Token
	ID     *StateID
	States []State
	Fault  []Action
}

func (*NestedState) stateNode()             {}
func (s *NestedState) TokenLiteral() string { return s.Token.Literal }

type SplitState struct {
	Token  token.Token
	ID     *StateID
	Splits []*StateID
	Fault  []Action
}

func (*SplitState) stateNode()             {}
func (s *SplitState) TokenLiteral() string { return s.Token.Literal }

type GotoAction struct {
	Token token.Token
	ID    *StateID
}

func (*GotoAction) actionNode()            {}
func (g *GotoAction) TokenLiteral() string { return g.Token.Literal }

type LogAction struct {
	Token   token.Token
	Message string
}

func (*LogAction) actionNode()            {}
func (a *LogAction) TokenLiteral() string { return a.Token.Literal }

type StateID struct {
	Token token.Token
	Name  string
}

func (s *StateID) TokenLiteral() string { return s.Token.Literal }

type SpawnAction struct {
	Token token.Token
	ID    *StateID
	Args  KeywordArgs
}

type KeywordArgs map[string]Expression

func (*SpawnAction) actionNode()            {}
func (a *SpawnAction) TokenLiteral() string { return a.Token.Literal }
