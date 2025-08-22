export enum MonitorType {
  Transaction = "Transaction",
  SpendingUTXOTransaction = "SpendingUTXOTransaction",
  RskPeginTransaction = "RskPeginTransaction",
  NewBlock = "NewBlock",
}
export type MonitorData = {
  [MonitorType.Transaction]?: string[];
  [MonitorType.SpendingUTXOTransaction]?: string[];
  [MonitorType.RskPeginTransaction]?: string[];
  [MonitorType.NewBlock]?: string[];
  [key: string]: any;
};

export function getMonitorType(monitorData: MonitorData): MonitorType {
  if (monitorData[MonitorType.Transaction]) {
    return MonitorType.Transaction;
  } else if (monitorData[MonitorType.SpendingUTXOTransaction]) {
    return MonitorType.SpendingUTXOTransaction;
  } else if (monitorData[MonitorType.RskPeginTransaction]) {
    return MonitorType.RskPeginTransaction;
  } else if (monitorData[MonitorType.NewBlock]) {
    return MonitorType.NewBlock;
  }

  // Default fallback
  return MonitorType.Transaction;
}

export function getMonitorTxid(monitorData: MonitorData): String {
  if (monitorData[MonitorType.Transaction]) {
    return monitorData[MonitorType.Transaction][0];
  }

  if (monitorData[MonitorType.SpendingUTXOTransaction]) {
    return monitorData[MonitorType.SpendingUTXOTransaction][0];
  }

  // Default fallback
  return "";
}

export function getMonitorByString(monitorString: String): MonitorType {
  if (monitorString === MonitorType.Transaction) {
    return MonitorType.Transaction;
  } else if (monitorString === MonitorType.SpendingUTXOTransaction) {
    return MonitorType.SpendingUTXOTransaction;
  } else if (monitorString === MonitorType.RskPeginTransaction) {
    return MonitorType.RskPeginTransaction;
  } else if (monitorString === MonitorType.NewBlock) {
    return MonitorType.NewBlock;
  }

  // Default fallback
  return MonitorType.Transaction;
}
