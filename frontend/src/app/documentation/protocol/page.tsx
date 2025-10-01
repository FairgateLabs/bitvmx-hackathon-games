export default function ProtocolPage() {
  return (
    <div className="mx-auto max-w-5xl px-4 py-10">
      <h1 className="text-3xl font-bold tracking-tight">
        Add Numbers Game Protocol and Program
      </h1>
      <p className="mt-4 text-muted-foreground">
        This document explains the program and protocol used for the Add Numbers
        game, a BitVMX-based game where two players compete to solve a
        mathematical challenge.
      </p>

      <section className="mt-10">
        <h2 className="text-2xl font-semibold">
          Dispute Resolution Protocol (DRP)
        </h2>
        <p className="mt-3 text-muted-foreground">
          The game uses BitVMX's dispute resolution protocol to ensure fair play
          and the Bitcoin network for decentralized immutable transactions.
        </p>

        <p className="mt-4 text-muted-foreground">
          The dispute resolution protocol is a{" "}
          <code className="rounded bg-muted px-1 py-0.5">
            Directed Acyclic Graphs
          </code>{" "}
          (DAGs) of bitcoin transactions between a prover (player 2) and a
          verifier (player 1) to determine if the execution of a program using{" "}
          <a
            href="https://github.com/FairgateLabs/BitVMX-CPU"
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-600 hover:text-blue-800 underline"
          >
            BitVMX-CPU
          </a>{" "}
          is correct or not, in a decentralized way with on chain commitments.
        </p>

        <p className="mt-4 text-muted-foreground">
          You can build your own protocols in BitVMX, to build the necessary{" "}
          <code className="rounded bg-muted px-1 py-0.5">DAG</code> you can use
          the{" "}
          <a
            href="https://github.com/FairgateLabs/rust-bitvmx-protocol-builder"
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-600 hover:text-blue-800 underline"
          >
            BitVMX Protocol Builder
          </a>
        </p>

        <div className="mt-6">
          <h3 className="text-xl font-semibold mb-4">
            Protocol Setup Requirements
          </h3>
          <p className="text-muted-foreground mb-4">
            Participants need to set up a protocol before using it, they need
            to:
          </p>
          <ul className="list-disc space-y-2 pl-6">
            <li>
              Create a musig2 multi signature address, we will call it{" "}
              <code className="rounded bg-muted px-1 py-0.5">
                agregated key
              </code>{" "}
              from now on.
            </li>
            <li>
              Fund the agregated key with an a{" "}
              <code className="rounded bg-muted px-1 py-0.5">
                external start utxo
              </code>
              , this funds will be used to pay for the bitcoin transaction fees
              needed to broadcast the DAGs transactions on chain in case of a
              challenge.
            </li>
            <li>
              Define an{" "}
              <code className="rounded bg-muted px-1 py-0.5">
                external action utxo
              </code>{" "}
              that will be locked in the agregated key and released when the
              challenge finishes.
            </li>
            <li>Sign the DAG's Bitcoin Transactions off-chain.</li>
          </ul>
        </div>

        <div className="mt-6">
          <h3 className="text-xl font-semibold mb-4">Protocol Flow</h3>
          <ol className="list-decimal space-y-2 pl-6">
            <li>
              First the verifier <strong>starts the challenge</strong>, when he
              thinks the other party obtained an incorrect answer.
            </li>
            <li>
              Then prover presents the parameter{" "}
              <code className="rounded bg-muted px-1 py-0.5">inputs</code> used
              to run the program.
            </li>
            <li>
              After that an N-ary search is run automatically by bitvmx, where
              the prover and verifier need to present the values of the search
              at each iteration to find the discrepancy between the execution
              traces.
            </li>
            <li>
              Once it's found, or if a participant don't present the proof
              before the timeout, the protocol declares a winner.
            </li>
          </ol>
        </div>

        <div className="mt-8">
          <h3 className="text-xl font-semibold mb-4">DRP Flow Diagram</h3>
          <p className="text-muted-foreground mb-4">
            The following diagram shows the complete DRP protocol flow with all
            the transactions and their relationships:
          </p>
          <div className="overflow-hidden rounded-lg border bg-background p-4">
            <img
              src="/protocol/protocol-flow.svg"
              alt="Protocol Flow Diagram"
              className="w-full"
            />
          </div>
        </div>
      </section>

      <section className="mt-12">
        <h2 className="text-2xl font-semibold">Program</h2>
        <p className="mt-3 text-muted-foreground">
          The program being used by add numbers game is{" "}
          <code className="rounded bg-muted px-1 py-0.5">add-test.elf</code>.
          The .elf is obtained through compilation of c++ or rust program with
          RISC V emulator.
        </p>

        <div className="mt-8">
          <h3 className="text-xl font-semibold mb-4">Code</h3>
          <p className="text-muted-foreground mb-4">
            The code of the program is:
          </p>
          <div className="overflow-hidden rounded-lg border bg-muted">
            <pre className="p-4 text-sm overflow-x-auto">
              <code className="language-cpp">{`#include <stdint.h>
#include "emulator.h"

int main(int x)
{
    unsigned int *a = (unsigned *)INPUT_ADDRESS;
    unsigned *b = a + 1;
    unsigned *c = a + 2;
    if (*a + *b == *c)
    {
        return 0;
    }
    else
    {
        return 1;
    }
}`}</code>
            </pre>
          </div>
        </div>

        <div className="mt-8">
          <h3 className="text-xl font-semibold mb-4">Yaml Configuration</h3>
          <p className="text-muted-foreground mb-4">
            In order to use it in BitVMX-CPU it needs more information than just
            the .elf, that's why we pass down a yaml file with the information
            needed. Like the number of equal parts to divide the array for the
            N-ary search algorithm, max number steps to perform and the inputs
            size and owner.
          </p>
          <p className="text-muted-foreground mb-4">
            This is the yaml used for this project:
          </p>
          <div className="overflow-hidden rounded-lg border bg-muted">
            <pre className="p-4 text-sm overflow-x-auto">
              <code className="language-yaml">{`elf: add-test.elf
nary_search: 8
max_steps: 50
input_section_name: .input
inputs:
- size: 8
    owner: const
- size: 4
    owner: prover`}</code>
            </pre>
          </div>
        </div>

        <div className="mt-8">
          <h3 className="text-xl font-semibold mb-4">Where is this set?</h3>
          <p className="text-muted-foreground mb-4">
            We define the .yaml program path when we send the{" "}
            <code className="rounded bg-muted px-1 py-0.5">
              IncomingBitVMXApiMessages::Setup
            </code>{" "}
            message to BitVMX client, we also set the program type to indicate
            the protocol we are going to use, in this case we will use the
            dispute resolution protocol (DRP).
          </p>
          <div className="overflow-hidden rounded-lg border bg-muted">
            <pre className="p-4 text-sm overflow-x-auto">
              <code className="language-rust">{`let program_id = Uuid::new_v4();
let program_path = "./verifiers/add-test-with-const-pre.yaml";
let participants =  vec![p2p_address_1, p2p_address_2];
let leader_idx = 0;

let response = self
    .rpc_client
    .send_request(IncomingBitVMXApiMessages::Setup(
        program_id,
        bitvmx_client::types::PROGRAM_TYPE_DRP,
        participants,
        leader_idx,
    ))
    .await?;`}</code>
            </pre>
          </div>
        </div>
      </section>
    </div>
  );
}
