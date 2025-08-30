import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useNetwork } from "@/hooks/useNetwork";
import { useNextGameState } from "@/hooks/useGameState";
import { NetworkType } from "@/types/network";

export function ChooseNetwork() {
  const { mutate: nextState } = useNextGameState();

  const handleNetworkSelect = async (network: NetworkType) => {
    nextState(null);
  };

  return (
    <div className="container mx-auto p-6 max-w-[840px]">
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl text-center">
            🔽 Select Network
          </CardTitle>
          <CardDescription className="text-center">
            Choose the Bitcoin network for your game session.
          </CardDescription>
        </CardHeader>
        <CardContent className="grid grid-cols-2 gap-4">
          <div className="flex flex-col sm:flex-row gap-4">
            <Button
              onClick={() => handleNetworkSelect(NetworkType.Regtest)}
              className="flex text-center h-40 text-lg"
              variant="outline"
            >
              <div className="flex flex-col w-full gap-2 h-[130px]">
                🔗 Regtest
                <p className="text-sm font-normal max-w-xs whitespace-normal block">
                  Easy to use and ideal for testing and development. Funds are
                  given to you automatically without worrying about funding your
                  wallet.
                </p>
              </div>
            </Button>
            <Button
              onClick={() => handleNetworkSelect(NetworkType.Testnet)}
              className="flex text-center h-40 text-lg"
              variant="outline"
              disabled={true}
            >
              <div className="flex flex-col w-full gap-2 h-[130px]">
                🔗 Testnet
                <p className="text-sm font-normal max-w-xs whitespace-normal block">
                  More complex and realistic. You need to fund your wallet
                  manually to play the game.
                </p>
              </div>
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
