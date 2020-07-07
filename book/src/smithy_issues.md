# Initial issues in Smithy Documentation

This is a rough list of issues I found while I was trying to develop another Smithy toolchain (not runtime).

# Specification Issues

A lot of the issues are really in the specification, there seem to be holes in it, maybe because of the assumption that the language will only have the one implementation. I think it would be valuable to determine, is it a description of the Smithy product, or is it a specification for a language; tool chain; approach, and if so is the current Smithy repo the reference implementation? Knowing this would help understand the role of the documentation and if it is in fact a specification. Once you have decided on the issues above it would be good to put the components in some perspective. An overview that describes the role of the Smithy language, the JSON representation, Smithy build, etc. and where you see a single implementation, where you see alternate implementations, and so forth. 

The rest of these notes make the following assumptions.

1. That there is a need for a language and runtime neutral descriptive language with well-described semantics and a flexible extension mechanism.
1. That there are two documented representations that have a hi-fidelity mapping to some abstract _Model_.
   1. It would be good to document this model, and the reasons why you may use the two different representations.
   1. That the current repo is a reference implementation of the model and reader/writer for the two representations.
1. That the intermediate model is a key API for shared tooling, not just the textual representatios.

## ABNF Errors

### Top-level structure

The first issue is actually that the complete ABNF has to be constructed from the in-specification fragments. It would be good to have the entire BNF as an appendix. Secondly there are some errors and inconsistencies that actually mean you have to scour the docs to make a determination.

The `idl` production in [section 2.1](https://awslabs.github.io/smithy/1.0/spec/core/lexical-structure.html#smithy-idl-abnf) appears to be in error:

```abnf
idl =
    ws
  / control_section
  / metadata_section
  / shape_section
```

Clearly it is not the case that a model file as a `control_section` _OR_ `metadata_section` _OR_ `shape_section`. The production should be:

```abnf
idl = ws control_section metadata_section shape_section
```

### Comments

Another issue in translating directly from the specification is the interaction between the `ws` ([section 2.1](https://awslabs.github.io/smithy/1.0/spec/core/lexical-structure.html#lexical-notes) and the `shape_documentation_comments` ([section 3.2](https://awslabs.github.io/smithy/1.0/spec/core/shapes.html#defining-shapes)) productions. Specifically the `ws` production is eager and will in general grab a comment such as "/// a doc comment" as a line comment with the value "/ a doc comment".

```abnf
ws =
    *(sp / newline / line_comment) ; whitespace

shape_documentation_comments =
    *(documentation_comment)
```

This took some experimentation in the PEG grammar [I developed](https://github.com/johnstonskj/rust-atelier/blob/master/atelier-smithy/src/smithy.pest), but using a look-ahead rule seemed most effective:

```text
ws =
    _{ (sp | (!("///") ~ line_comment) | NEWLINE)* }
```

### Identifiers

The `identifier` production has an issue in that it allows for the strings "_" and "_123" as legal identifiers which I assume is incorrect.

```abnf
identifier =
    (ALPHA / "_") *(ALPHA / DIGIT / "_")
```

### Namespace cardinality?

3. is there just one namespace, or not?
   * the bnf in §3.1 seems to say no...
   * [https://awslabs.github.io/smithy/1.0/guides/style-guide.html#one-namespace-per-file] seems unsure
   * see §20 - merging models, this doesn't say
   * JSON AST, but these could, but do they?

## Process Comments

1. why/when models merge
1. splitting a model over 
1. how do multiple "models" interact?

# Missing

ABNF appendix

The rest of the prelude would be nice

How are models merged, how is tooling expected to work? it seems to be, go check the smithy-build code.
