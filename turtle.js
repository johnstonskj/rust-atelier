/*
Language: RDF Turtle
Author: Simon Johnston <johnstonskj@gmail.com>
Website: https://www.w3.org/TR/turtle/
Category: rdf
*/

hljsTurtle = function(hljs) {
  return {
    name: 'Turtle',
    keywords: {
      keyword: '@prefix PREFIX @base BASE',
      literal: 'a true false',
    },
    contains: [
      {
        className: 'title',
        begin: '<',
        end: '>',
        illegal: '\\n'
      },
      {
        className: 'meta',
        begin: '\\w*:[\t \r\n$]'
      },
      {
        className: 'function',
        begin: '(\\w+:\\w+)|(:\\w+)|(_:\\w+)'
      },
      {
        className: 'string',
        begin: '"',
        end: '"(\\^\\^|@\\w+(\\-\\w+)*)?',
        illegal: '\\n',
        contains: [hljs.BACKSLASH_ESCAPE]
      },
      {
        className: 'number',
        variants: [
          { begin: '\\b([+-]?\\d[\\d]*(\\.[0-9]+)?([eE][+-]?[0-9]+)?)' }
        ],
        relevance: 0
      },
      {
        className: 'comment',
        begin: '#',
        end: '$'
      }
    ]
  };
}

// module.exports = hljsTurtle;
