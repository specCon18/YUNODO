# YUNODO(Why you no do?)
I needed a tool to find all of my "//TODO:"s in my code so I wrote this.

## Q&A
**Q**:is it over engineered?
**A**:yes. yes it is.

**Q**:do I care that it's over engineered?
**A**:no. no I don't.

**Q**:should you have that it's over engineered?
**A**:no if you don't like it gon' now git!

## Purpose
take comments like the following and output them in a parser/reader friendly format.
#### Input:
`//TODO:this is a todo comment:ODOT//`

#### Output:
| File Path | File Name | Line Number | Comment |
|:----------|:---------:|:-----------:|:--------|
| ./src/ | README.md | 17 |  this is a todo comment  |
