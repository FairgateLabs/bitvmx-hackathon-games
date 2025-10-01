import {
  FileText,
  GraduationCap,
  Calculator,
  Gamepad2,
  ArrowLeftRight,
  Code2,
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
    title: "Documentation",
    icon: GraduationCap, // más didáctico que BookOpen
    items: [
      {
        title: "Game Flow",
        url: "/documentation/game-flow",
        icon: FileText, // más claro para docs
      },
      {
        title: "Game Play",
        url: "/documentation/game-play",
        icon: Gamepad2, // acción de jugar
      },
      {
        title: "Protocol & Program",
        url: "/documentation/protocol",
        icon: Code2, // código y protocolo
      },
    ],
  },
  {
    title: "Add Numbers",
    icon: Calculator, // comunica sumas directamente
    items: [
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
