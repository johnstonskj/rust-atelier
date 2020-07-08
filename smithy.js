/*
Language: Smithy
Author: Simon Johnston <johnstonskj@gmail.com>
Website: https://awslabs.github.io/smithy/
Category: idl
*/

hljsSmithy = function(hljs) {
  var NAMESPACE_RE = /[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*/;
  var IDENT_RE = /[a-zA-Z_][a-zA-Z0-9_]*/;
  var SHAPE_ID_RE = /([a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*#)?([a-zA-Z_][a-zA-Z0-9_]*)(\$[a-zA-Z_][a-zA-Z0-9_]*())?/
  var SIMPLE =
    'bigDecimal bigInteger blob boolean byte document double float integer long short string timestamp';
  var SHAPES =
    'list set map structure union service operation resource';
  var KEYWORDS =
    'namespace use apply';
    ;
  return {
    name: 'Smithy',
    keywords: {
      keyword:
        KEYWORDS,
      literal:
        'true false null',
    },
    contains: [
      hljs.C_LINE_COMMENT_MODE,
      hljs.inherit(hljs.QUOTE_STRING_MODE, {begin: /b?"/, illegal: null}),
      {
        className: 'string',
        variants: [
           { begin: /r"(.|\n)*?"/ },
           { begin: /r"""(.|\n)*?"""/ }
        ],
        contains: [hljs.BACKSLASH_ESCAPE]
      },
      {
        className: 'number',
        variants: [
          { begin: '\\b(\\d[\\d_]*(\\.[0-9_]+)?([eE][+-]?[0-9_]+)?)' }
        ],
        relevance: 0
      },
      // Traits
      {
        className: 'meta',
        begin: /@[^\t \r\n\(]+/
      },
      // Namespace statement
      {
        className: 'keyword',
        beginKeywords: 'namespace',
        end: /$/,
        contains: [
          {
            className: 'title',
            begin: NAMESPACE_RE,
            endsParent: true
          }
        ]
      },
      // Other statements
      {
        className: 'keyword',
        beginKeywords: 'apply use',
        end: /[\t \r\n$]/,
      },
      // Simple Shapes
      {
        className: 'class',
        beginKeywords: SIMPLE,
        end: /$/,
        contains: [
          {
            className: 'title',
            begin: IDENT_RE,
            endsParent: true
          }
        ]
      },
      // Complex Shapes
      {
        className: 'class',
        beginKeywords: SHAPES, end: '{',
        contains: [
          {
            className: 'title',
            begin: IDENT_RE,
            endsParent: true
          }
        ]
      },
      // Complex shape references?
      {
        className: 'name',
        begin: /:[\t ]*/,
        excludeBegin: true,
        contains: [
          {
            className: 'title',
            begin: IDENT_RE,
            endsParent: true
          }
        ]
      },
    ]
  };
}

// module.exports = hljsSmithy;
