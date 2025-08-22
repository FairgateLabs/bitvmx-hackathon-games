import { useQuery } from "@tanstack/react-query";

const fetchPrograms = async () => {
  try {
    const response = await fetch(`http://localhost:8080/program/list`);
    const monitor = await response.json();
    return monitor;
  } catch (error) {
    throw new Error("Error fetching programs: " + error);
  }
};

export type Program = {
  program_id: string;
};

const usePrograms = () => {
  return useQuery<Program[], Error>({
    queryKey: ["programs"],
    queryFn: fetchPrograms,
  });
};

export default usePrograms;
