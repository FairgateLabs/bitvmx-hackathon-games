import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { CopyButton } from "@/components/ui/copy-button";
import { Loader } from "@/components/ui/loader";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { useSaveFundingUtxos } from "@/hooks/useFundingUtxos";
import { EnumPlayerRole } from "@/types/game";
import { Utxo } from "../../../../backend/bindings/Utxo";
import { useCurrentGame } from "@/hooks/useGame";
import { useQueryClient } from "@tanstack/react-query";

export function FundingExchange({ expand = true }) {
  const [isOpen, setIsOpen] = useState(expand);
  const [jsonInput, setJsonInput] = useState("");
  const [jsonError, setJsonError] = useState("");
  const { data: currentGame } = useCurrentGame();
  const queryClient = useQueryClient();
  const { mutate: saveFundingUtxos, isPending } = useSaveFundingUtxos();

  const isValidTxid = (txid: string): boolean => {
    const hexRegex = /^[0-9a-fA-F]{64}$/;
    return hexRegex.test(txid);
  };

  const isValidAmount = (amount: bigint): boolean => {
    return amount > BigInt(0);
  };

  const isValidUtxo = (utxo: Utxo): boolean => {
    return (
      utxo &&
      isValidTxid(utxo.txid) &&
      utxo.vout !== undefined &&
      utxo.amount !== undefined &&
      isValidAmount(utxo.amount)
    );
  };

  const handleJsonPaste = (jsonString: string) => {
    setJsonInput(jsonString);
    setJsonError("");

    try {
      const funding_parsed = JSON.parse(jsonString);

      if (
        isValidUtxo(funding_parsed.funding_protocol_utxo) &&
        isValidUtxo(funding_parsed.funding_bet_utxo)
      ) {
        setJsonError("");
      } else {
        setJsonError(
          "Invalid UTXO format. Missing required fields: txid, vout, amount"
        );
      }
    } catch {
      setJsonError("Invalid JSON format");
    }
  };

  useEffect(() => {
    if (isPending) {
      queryClient.invalidateQueries({ queryKey: ["currentGame"] });
    }
  }, [isPending]);

  const handleSendOtherUtxo = () => {
    saveFundingUtxos({
      program_id: currentGame?.program_id || "",
      funding_protocol_utxo: JSON.parse(jsonInput).funding_protocol_utxo,
      funding_bet_utxo: JSON.parse(jsonInput).funding_bet_utxo,
    });
  };

  const getMyUtxoJson = () => {
    const funding_data = currentGame?.bitvmx_program_properties;
    if (!funding_data?.funding_protocol_utxo) return "";

    return JSON.stringify(
      {
        funding_protocol_utxo: funding_data.funding_protocol_utxo,
        funding_bet_utxo: funding_data.funding_bet_utxo,
      },
      null,
      2
    );
  };

  const isJsonValid = (jsonInput: string) => {
    try {
      const parsed = JSON.parse(jsonInput);
      return (
        isValidUtxo(parsed.funding_protocol_utxo) &&
        isValidUtxo(parsed.funding_bet_utxo)
      );
    } catch {
      return false;
    }
  };

  return (
    <div className="p-4 border rounded-lg">
      <Collapsible open={isOpen} onOpenChange={setIsOpen}>
        <CollapsibleTrigger asChild>
          <h3 className="font-semibold mb-3 cursor-pointer">
            💰 Bitcoin UTXO Exchange
          </h3>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <p className="text-sm mb-4">
            {currentGame?.role === EnumPlayerRole.Player1
              ? "Share your UTXO information with Player 2, this will be used to fund the game and the bet."
              : "Copy Player 1 UTXO information and paste it here, this will be used to fund the game and the bet."}
          </p>

          {currentGame?.bitvmx_program_properties.funding_protocol_utxo && (
            <>
              <div className="rounded-lg">
                <div className="flex items-center justify-between mb-2">
                  <h4 className="font-semibold mb-3">
                    Funding Protocol and Bet UTXO
                  </h4>
                  {currentGame?.role === EnumPlayerRole.Player1 && (
                    <CopyButton
                      text={getMyUtxoJson()}
                      size="sm"
                      variant="outline"
                    >
                      Copy to Share
                    </CopyButton>
                  )}
                </div>

                <pre className="w-full h-50 p-2 text-xs font-mono border rounded resize-none overflow-auto">
                  {JSON.stringify(JSON.parse(getMyUtxoJson()), null, 2)}
                </pre>
              </div>
            </>
          )}

          {!currentGame?.bitvmx_program_properties.funding_protocol_utxo &&
            currentGame?.role === EnumPlayerRole.Player2 && (
              <div className="p-4">
                <h4 className="font-semibold mb-3">
                  Other Player&apos;s Protocol and Bet UTXO Information
                </h4>

                <div className="mb-4">
                  <textarea
                    value={jsonInput}
                    rows={10}
                    onChange={(e) => handleJsonPaste(e.target.value)}
                    placeholder='Paste JSON here, e.g., {"txid":"123...","vout":0,"amount":1000,"output_type":{}}'
                    className="w-full  p-2 text-xs font-mono border rounded resize-none"
                  />
                  {jsonError && (
                    <p className="text-sm mt-1 text-red-600">{jsonError}</p>
                  )}
                  {!jsonError && isJsonValid(jsonInput) && (
                    <p className="text-sm mt-1 text-green-600">
                      ✅ Valid UTXO JSON
                    </p>
                  )}
                </div>

                <Button
                  onClick={handleSendOtherUtxo}
                  disabled={!isJsonValid(jsonInput) || isPending}
                  className="w-full"
                >
                  <span className="flex items-center justify-center gap-2">
                    {isPending ? (
                      <>
                        <Loader />
                        Saving...
                      </>
                    ) : (
                      "📤 Send Other Player's UTXO"
                    )}
                  </span>
                </Button>
              </div>
            )}
          {!isPending && (
            <div className="p-4 mt-3 bg-yellow-50 border border-yellow-200 rounded-lg">
              <h4 className="font-semibold text-yellow-800">
                ⚠️ UTXO Exchange Required
              </h4>
              <p className="text-sm text-yellow-700">
                Both players must generate and share their UTXO information to
                continue with the game. <br />
                In this game, Player 2 should copy the UTXO information from
                Player 1 and paste it into their UTXO form.
              </p>
            </div>
          )}
        </CollapsibleContent>
      </Collapsible>
    </div>
  );
}
