import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { useNextGameState } from "@/hooks/useGameState";
import { useSaveGameRole } from "@/hooks/useGameRole";
import { GameState, PlayerRole } from "@/types/game";
import { useEffect, useState } from "react";

interface GameRoleSelectorProps {
  title: string;
  description: string;
  subtitle: string;
}

export function ChooseRole({
  title,
  description,
  subtitle,
}: GameRoleSelectorProps) {
  const { mutate: nextState } = useNextGameState();
  const { mutate: saveRole } = useSaveGameRole();
  const [currentPort, setCurrentPort] = useState<number | null>(null);

  useEffect(() => {
    // Get the current port from window.location.port or environment
    const port = window.location.port ? parseInt(window.location.port) : 3000;
    setCurrentPort(port);
  }, []);

  const handleRoleSelect = (role: PlayerRole) => {
    saveRole(role);
    nextState(GameState.ChooseNetwork);
  };

  // Determine which role should be enabled based on port
  const isPlayer1Enabled = currentPort === 3000;
  const isPlayer2Enabled = currentPort === 3001;

  return (
    <div className="container mx-auto p-6 max-w-2xl">
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl text-center">{title}</CardTitle>
          <CardDescription className="text-center">
            {description}
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="text-center mb-6">
            <p className="text-muted-foreground">{subtitle}</p>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <Button
              onClick={() => handleRoleSelect(PlayerRole.Player1)}
              className="h-24 text-lg"
              variant="outline"
              disabled={!isPlayer1Enabled}
            >
              ➕ Player 1<br />
              <span className="text-sm font-normal">Create the game</span>
            </Button>

            <Button
              onClick={() => handleRoleSelect(PlayerRole.Player2)}
              className="h-24 text-lg"
              variant="outline"
              disabled={!isPlayer2Enabled}
            >
              🤝 Player 2<br />
              <span className="text-sm font-normal">Join the game</span>
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
