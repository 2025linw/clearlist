export default {
  meta: {
    type: 'suggestion',
    docs: {
      description: 'Disallow single-element React Native style arrays',
    },
    fixable: 'code',
    schema: [],
    messages: {
      singleStyleArray: 'Avoid single-element style arrays. Use style={...} instead.',
    },
  },

  create(context) {
    const sourceCode = context.sourceCode;

    return {
      JSXAttribute(node) {
        if (node.name.name !== 'style') return;

        const expr = node.value?.expression;

        if (expr?.type !== 'ArrayExpression') return;
        if (expr.elements.length !== 1) return;
        if (!expr.elements[0]) return;

        context.report({
          node,
          messageId: 'singleStyleArray',
          fix(fixer) {
            const inner = sourceCode.getText(expr.elements[0]);
            return fixer.replaceText(node.value, `{${inner}}`);
          },
        });
      },
    };
  },
};
