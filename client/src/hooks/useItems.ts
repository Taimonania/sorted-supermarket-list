"use client";

import useSWR from "swr";
import getAxiosInstance from "../axiosInstance";

export default function useItems() {
  const { data, error, mutate } = useSWR("/items", swrFetcher);
  return {
    items: data,
    isLoading: !data,
    isError: !!error,
    mutate,
  };
}

function swrFetcher(url: string) {
  const axiosInstance = getAxiosInstance();
  return axiosInstance.get(url).then((res) => res.data);
}
