type EventCallback = (data: unknown) => void;

class RealtimeClient {
  private ws: WebSocket | null = null;
  private listeners = new Map<string, Set<EventCallback>>();
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 10;
  private shouldReconnect = false;
  private token = "";

  connect(token: string) {
    this.token = token;
    this.shouldReconnect = true;
    this.reconnectAttempts = 0;
    this.createConnection();
  }

  disconnect() {
    this.shouldReconnect = false;
    this.ws?.close();
    this.ws = null;
  }

  on(eventType: string, cb: EventCallback) {
    if (!this.listeners.has(eventType)) {
      this.listeners.set(eventType, new Set());
    }
    this.listeners.get(eventType)!.add(cb);
  }

  off(eventType: string, cb: EventCallback) {
    this.listeners.get(eventType)?.delete(cb);
  }

  private createConnection() {
    this.ws = new WebSocket(`ws://localhost:5000/ws?token=${this.token}`);

    this.ws.onopen = () => {
      console.log("[WS_CLIENT]: connected");
      this.reconnectAttempts = 0;
    };

    this.ws.onmessage = (event: MessageEvent) => {
      try {
        const data = JSON.parse(event.data);
        const eventType = data.event_type as string;
        console.log("[WS_CLIENT]: received event_type=", eventType, data);
        if (eventType && this.listeners.has(eventType)) {
          for (const cb of this.listeners.get(eventType)!) {
            cb(data);
          }
        }
      } catch {
        // ignore malformed messages
      }
    };

    this.ws.onclose = (e) => {
      console.warn("[WS_CLIENT]: closed", e.code, e.reason);
      if (this.shouldReconnect) {
        this.scheduleReconnect();
      }
    };

    this.ws.onerror = () => {
      console.error("[WS_CLIENT]: error");
      this.ws?.close();
    };
  }

  private scheduleReconnect() {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) return;
    const delay = Math.min(1000 * 2 ** this.reconnectAttempts, 30000);
    this.reconnectAttempts++;
    setTimeout(() => this.createConnection(), delay);
  }
}

export const realtime = new RealtimeClient();
