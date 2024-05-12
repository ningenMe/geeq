import { Configuration, DefaultApi } from "../generated";

const config = new Configuration({
  basePath: process.env.NEXT_PUBLIC_API_ORIGIN,
})
export const geeqApiClient = new DefaultApi(config);
