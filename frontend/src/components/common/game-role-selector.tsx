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
import { GameState } from "@/types/gameState";

export enum GameRole {
  Player1 = "Player 1",
  Player2 = "Player 2",
}

export type Role = GameRole.Player1 | GameRole.Player2 | null;

interface GameRoleSelectorProps {
  title?: string;
  description?: string;
  subtitle?: string;
}

export function ChooseRole({
  title = "🎮 Add Numbers Game",
  description = "Choose the role you want to play",
  subtitle = "Two players compete by adding numbers. Who are you?",
}: GameRoleSelectorProps) {
  const { mutate: nextState } = useNextGameState();
  const { mutate: saveRole } = useSaveGameRole();

  const handleRoleSelect = (role: GameRole) => {
    saveRole(role);
    nextState(GameState.ChooseNetwork);
  };

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
              onClick={() => handleRoleSelect(GameRole.Player1)}
              className="h-24 text-lg"
              variant="outline"
            >
              ➕ Player 1<br />
              <span className="text-sm font-normal">Create the game</span>
            </Button>

            <Button
              onClick={() => handleRoleSelect(GameRole.Player2)}
              className="h-24 text-lg"
              variant="outline"
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
