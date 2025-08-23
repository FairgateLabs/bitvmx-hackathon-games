import {
  BookOpen,
  FileText,
  GraduationCap,
  Calculator,
  Plus,
  Gamepad2,
  Shuffle,
  AppWindow,
  Grid3x3,
  ArrowLeftRight,
} from "lucide-react";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubItem,
} from "@/components/ui/sidebar";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@radix-ui/react-collapsible";
import Link from "next/link";

// Menu items.

const menu_items = [
  {
    title: "Learning BitVMX",
    icon: GraduationCap, // más didáctico que BookOpen
    items: [
      {
        title: "Documentation",
        url: "/learning-bitvmx/documentation",
        icon: FileText, // más claro para docs
      },
    ],
  },
  {
    title: "Add Numbers",
    icon: Calculator, // comunica sumas directamente
    items: [
      {
        title: "How to play",
        url: "/add-numbers/how-to-play",
        icon: BookOpen, // tutorial/guide
      },
      {
        title: "Play game",
        url: "/add-numbers/play-game",
        icon: Gamepad2, // acción de jugar
      },
      {
        title: "Game Transactions",
        url: "/add-numbers/transactions",
        icon: ArrowLeftRight, // flujo de BTC/fondos
      },
    ],
  },
  {
    title: "Tic Tac Toe",
    icon: Grid3x3, // perfecto para el tablero
    items: [
      {
        title: "How to play",
        url: "tic-tac-toe/how-to-play",
        icon: BookOpen,
      },
      {
        title: "Play game",
        url: "tic-tac-toe/play-game",
        icon: Gamepad2,
      },
      {
        title: "Game Transactions",
        url: "/tic-tac-toe/transactions",
        icon: ArrowLeftRight,
      },
    ],
  },
];

export function AppSidebar() {
  return (
    <Sidebar>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>
            <Link href="/" className="text-lg">
              BitVMX Berlin Hackathon
            </Link>
          </SidebarGroupLabel>
          <SidebarGroupContent className="mt-4">
            <SidebarMenu>
              {menu_items.map((item) => (
                <SidebarMenuItem key={item.title}>
                  {item.items ? (
                    <Collapsible defaultOpen className="group/collapsible">
                      <CollapsibleTrigger asChild>
                        <SidebarMenuButton>
                          <item.icon />
                          <span>{item.title}</span>
                        </SidebarMenuButton>
                      </CollapsibleTrigger>
                      <CollapsibleContent>
                        <SidebarMenuSub>
                          {item.items.map((subItem) => (
                            <SidebarMenuSubItem key={subItem.title}>
                              <SidebarMenuButton asChild>
                                <Link href={subItem.url}>
                                  <subItem.icon />
                                  <span>{subItem.title}</span>
                                </Link>
                              </SidebarMenuButton>
                            </SidebarMenuSubItem>
                          ))}
                        </SidebarMenuSub>
                      </CollapsibleContent>
                    </Collapsible>
                  ) : (
                    <SidebarMenuButton asChild>
                      {/* <a href={item.url ?? "#"}>
                        <item.icon />
                        <span>{item.title}</span>
                      </a> */}
                    </SidebarMenuButton>
                  )}
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  );
}
