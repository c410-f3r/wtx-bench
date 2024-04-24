import cssnano from 'cssnano';
import pruneVar from 'postcss-prune-var';
import purgecss from '@fullhuman/postcss-purgecss';
import variableCompress from 'postcss-variable-compress';

export default {
  plugins: [
    cssnano({
      preset: 'default'
    }),
    pruneVar(),
    purgecss({
      content: ['**/*.html', '**/*.svelte']
    }),
    variableCompress()
  ]
};
