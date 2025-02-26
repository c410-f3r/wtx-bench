import cssnano from "cssnano";
import pruneVar from "postcss-prune-var";
import { purgeCSSPlugin } from "@fullhuman/postcss-purgecss";
import variableCompress from "postcss-variable-compress";

export default {
  plugins: [
    cssnano({
      preset: "default",
    }),
    pruneVar(),
    purgeCSSPlugin({
      content: ["**/*.html", "**/*.svelte"],
    }),
    variableCompress(),
  ],
};
