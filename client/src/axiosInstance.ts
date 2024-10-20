import axios, { AxiosInstance } from "axios";

let axiosInstance: AxiosInstance | null = null;

const getAxiosInstance = (): AxiosInstance => {
  if (!axiosInstance) {
    axiosInstance = axios.create({
      baseURL: process.env.NEXT_PUBLIC_API_BASE_URL,
    });
  }
  return axiosInstance;
};

export default getAxiosInstance;
