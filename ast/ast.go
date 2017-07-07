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
func (s *GotoAction) TokenLiteral() string { return s.Token.Literal }

type LogAction struct {
	Token   token.Token
	Message string
}

func (*LogAction) actionNode()            {}
func (s *LogAction) TokenLiteral() string { return s.Token.Literal }

type StateID struct {
	Token token.Token
	Name  string
}

func (s *StateID) TokenLiteral() string { return s.Token.Literal }
