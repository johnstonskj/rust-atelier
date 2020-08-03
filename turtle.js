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
      keyword: 'PREFIX BASE', // SPARQL style
      literal: 'a true false',
    },
    contains: [
      {
        className: 'keyword', // N3 style
        begin: /@[^\t \r\n\(]+/
      },
      {
        className: 'title',   // IRIs
        begin: '<',
        end: '>',
        illegal: '\\n'
      },
      {
        className: 'meta',    // only valid in PREFIX/BASE
        begin: '\\w*:[\t \r\n$]'
      },
      {
        className: 'function', // QName
        begin: '(\\w+:\\w+)|(:\\w+)|(_:\\w+)'
      },
      {
        className: 'string',  // RDFString literals
        begin: '"',
        end: '"(\\^\\^|@\\w+(\\-\\w+)*)?',
        illegal: '\\n',
        contains: [hljs.BACKSLASH_ESCAPE]
      },
      {
        className: 'number',  // numeric literals
        variants: [
          { begin: '\\b([+-]?\\d[\\d]*(\\.[0-9]+)?([eE][+-]?[0-9]+)?)' }
        ],
        relevance: 0
      },
      {
        className: 'comment', // N3 style
        begin: '#',
        end: '$'
      }
    ]
  };
}

// module.exports = hljsTurtle;
