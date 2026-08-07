import { env } from '$env/dynamic/private';

const apiBase = env.API_INTERNAL_URL || 'http://127.0.0.1:9000';

export async function apiRequest<T>(path: string, init: RequestInit = {}): Promise<T> {
  const response = await fetch(`${apiBase}${path}`, {
    ...init,
    headers: {
      accept: 'application/json',
      ...(init.body ? { 'content-type': 'application/json' } : {}),
      ...init.headers
    }
  });

  const payload = await response.json().catch(() => null);
  if (!response.ok) {
    throw new Error(payload?.error || `API request failed (${response.status})`);
  }
  return payload as T;
}
