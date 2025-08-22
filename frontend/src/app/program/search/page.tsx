"use client";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { Input } from "@/components/ui/input";
import useProgram from "@/hooks/useProgram";
import Link from "next/link";
import { useEffect, useState } from "react";

export default function ProgramSearch() {
  const [programId, setProgramId] = useState<string>("");
  const { data: program, isLoading } = useProgram(programId);

  console.log("program", program);
  console.log("isLoading", isLoading);
  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const id = searchParams.get("id");
    if (id) {
      setProgramId(id);
    }
  }, [programId]);

  return (
    <div className="flex flex-col min-h-screen p-8 pb-20 gap-8 sm:p-20 font-[family-name:var(--font-geist-sans)] max-w-[1000px] w-full overflow-scroll">
      <h1 className="text-4xl font-bold">Search Program</h1>

      <Input
        type="text"
        value={programId}
        onChange={(e) => setProgramId(e.target.value)}
        placeholder="Enter Program ID"
      />
      {program && (
        <div className="flex flex-col gap-4">
          <div className="flex gap-4">
            <span className="font-bold">Program ID:</span>
            <span className="font-mono">{program.program_id}</span>
          </div>
          <div className="flex gap-4">
            <span className="font-bold">My Index:</span>
            <span className="font-mono">{program.my_idx}</span>
          </div>
          <div className="flex gap-4">
            <span className="font-bold">State:</span>
            <span className="font-mono">{program.state}</span>
          </div>

          <div className="flex flex-col">
            <div className="font-bold">Protocol:</div>
            <div className="font-mono ml-4">
              <div className="mr-4">
                <span className="font-bold">Name:</span>
                <span className="font-mono">
                  <Link
                    href={`/protocol/search?id=${
                      program.protocol.LockProtocol?.ctx.protocol_name ||
                      program.protocol.DisputeResolutionProtocol?.ctx
                        .protocol_name
                    }`}
                    className="font-mono truncate hover:underline cursor-pointer"
                  >
                    {program.protocol.LockProtocol?.ctx.protocol_name ||
                      program.protocol.DisputeResolutionProtocol?.ctx
                        .protocol_name}
                  </Link>
                </span>
              </div>
              <div>
                <span className="font-bold">ID:</span>{" "}
                <span className="font-mono">
                  {program.protocol.LockProtocol?.ctx.id ||
                    program.protocol.DisputeResolutionProtocol?.ctx.id}
                </span>
              </div>
            </div>
          </div>

          <div className="flex gap-4 mt-6">
            <span className="font-bold">
              {program.participants.length} Participants:
            </span>
          </div>

          <Accordion type="single" collapsible className="ml-4 ">
            {program.participants.map((participant, index) => (
              <AccordionItem key={index} value={`item-${index}`}>
                <AccordionTrigger className="cursor-pointer">
                  Participant {index + 1}
                </AccordionTrigger>
                <AccordionContent
                  key={`item-${index}`}
                  className="ml-4 p-2 border-l-2 border-gray-200"
                >
                  <div className="flex flex-col gap-4">
                    <div>
                      <span className="font-bold">P2P Address:</span>{" "}
                      <span className="font-mono">
                        {participant.p2p_address.address}
                      </span>
                    </div>
                    <div>
                      <span className="font-bold">Peer ID:</span>{" "}
                      <pre className="font-mono text-xs bg-gray-100 p-1 rounded mt-1">
                        {JSON.stringify(participant.p2p_address.peer_id)}
                      </pre>
                    </div>
                    <div>
                      <span className="font-bold">Keys:</span>
                      <Accordion
                        type="single"
                        collapsible
                        className="list-disc ml-5"
                      >
                        {Object.entries(participant.keys.mapping).map(
                          (data) => (
                            <AccordionItem key={data[0]} value={data[0]}>
                              <AccordionTrigger className="cursor-pointer">
                                {data[0]}
                              </AccordionTrigger>
                              <AccordionContent className="font-mono text-xs bg-gray-100 p-1 rounded mt-1">
                                {JSON.stringify(data[1], null, 2)}
                              </AccordionContent>
                            </AccordionItem>
                          )
                        )}
                      </Accordion>
                    </div>
                    <div>
                      <span className="font-bold">Public Nonces:</span>
                      <pre className="font-mono text-xs bg-gray-100 p-1 rounded mt-1">
                        {JSON.stringify(participant.nonces, null, 2)}
                      </pre>
                    </div>
                    <div>
                      <span className="font-bold">Partial Signatures:</span>
                      <pre className="font-mono text-xs bg-gray-100 p-1 rounded mt-1">
                        {JSON.stringify(participant.partial, null, 2)}
                      </pre>
                    </div>
                  </div>
                </AccordionContent>
              </AccordionItem>
            ))}
          </Accordion>
        </div>
      )}
      {isLoading && <div className="text-gray-500">Loading...</div>}
      {!isLoading && !program && programId && (
        <div className="text-red-500">Program not found!</div>
      )}
    </div>
  );
}
