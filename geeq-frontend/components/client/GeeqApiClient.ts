import { Configuration, DefaultApi } from "../generated";

const config = new Configuration({
  basePath: "http://localhost:50051"
})
export const geeqApiClient = new DefaultApi(config);
