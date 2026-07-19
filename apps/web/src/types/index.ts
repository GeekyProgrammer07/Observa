export interface Monitor {
  id: string;
  url: string;
  name: string | null;
  interval: number;
  timeout_ms: number;
  is_paused: boolean;
  created_at: string;
}

export interface NotificationChannel {
  id: string;
  channel_type: "Email" | "Webhook";
  value: string;
  verified: boolean;
  created_at: string;
}

export interface User {
  id: string;
  username: string;
}

export interface AuthTokens {
  access_token: string;
  token_type: string;
  expires_in: number;
}
