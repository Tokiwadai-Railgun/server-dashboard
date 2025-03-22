// routes/api/download/[filename]/+server.ts
import { API_URL } from '$env/static/private';
import type { Cookies } from '@sveltejs/kit';

export async function GET({ params, cookies, request }: { params: any, cookies: Cookies, request: Request }) {
  const token = cookies.get("session_token");
  if (!token) {
    return new Response("Unauthorized", { status: 401 });
  }
  
  try {
    const response = await fetch(`${API_URL}/storage/download/${params.filename}`, {
      headers: {
        "Authorization": token,
        "user_id": "1" // TODO: Remove hardcoded user_id
      }
    });
    
    if (!response.ok) {
      return new Response(`API error: ${response.statusText}`, { 
        status: response.status 
      });
    }
    
    // Forward the response headers and body
    const headers = new Headers();
    response.headers.forEach((value, key) => {
      headers.set(key, value);
    });

    headers.set("Content-Disposition", `attachment; filename="${params.filename}"`)
    
    return new Response(response.body, {
      status: 200,
      headers
    });
  } catch (error: any) {
    return new Response(`Error: ${error.message}`, { status: 500 });
  }
}
