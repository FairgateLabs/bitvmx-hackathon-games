import { Home, Inbox, ArrowLeftRight, BookOpen } from "lucide-react";

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
    title: "Add Numbers",
    icon: Home,
    items: [
      {
        title: "How to Play",
        url: "/add-numbers/how-to-play",
        icon: BookOpen,
      },
      {
        title: "Play",
        url: "/add-numbers",
        icon: ArrowLeftRight,
      },
    ],
  },
  {
    title: "Tic Tac Toe",
    icon: Inbox,
    items: [
      {
        title: "Coming soon",
        url: "#",
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
          <SidebarGroupLabel>BitVMX Games</SidebarGroupLabel>
          <SidebarGroupContent>
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
