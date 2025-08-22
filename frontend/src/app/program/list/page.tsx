"use client";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import usePrograms, { Program } from "@/hooks/usePrograms";
import Link from "next/link";

export default function ProgramList() {
  const { data: programs, isLoading } = usePrograms();

  return (
    <div className="flex flex-col min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <h1 className="text-4xl font-bold">Programs</h1>

      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Program ID</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {programs?.map((program: Program) => (
            <TableRow key={program.program_id}>
              <TableCell className="font-mono font-bold text-left">
                <Link
                  href={`/program/search?id=${program.program_id}`}
                  className="cursor-pointer"
                >
                  {program.program_id}
                </Link>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
