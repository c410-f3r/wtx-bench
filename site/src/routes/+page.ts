import { resolve } from "$app/paths";
import Csv from "$lib/Csv";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch }) => {
  const response = await fetch(resolve("/report.csv.gzip", {}));
  return {
    csv: new Csv(await decompress_gzip(response)),
  };
};

const decompress_gzip = async (response: Response) => {
  const ds = new DecompressionStream("gzip");
  const response_blob = await response.blob();
  const decoded_blob = await new Response(
    response_blob.stream().pipeThrough(ds),
  ).blob();
  return await decoded_blob.text();
};
