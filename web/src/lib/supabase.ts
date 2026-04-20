import { createClient } from "@supabase/supabase-js";

const supabaseUrl = "https://tblyrllukumgejwjvqld.supabase.co";
const supabaseAnonKey =
  "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InRibHlybGx1a3VtZ2Vqd2p2cWxkIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NzM3OTMyODgsImV4cCI6MjA4OTM2OTI4OH0.SENR4DAH6gI-VOhFcUYTwUqW_RmCrKPGYbeDWBb7r08";

export const supabase = createClient(supabaseUrl, supabaseAnonKey);

export async function joinWaitlist(email: string): Promise<{ success: boolean; message: string }> {
  try {
    const { error } = await supabase
      .from("waitlist")
      .insert([{ email, source: "lesearch-website" }]);

    if (error) {
      // Duplicate email — unique constraint violation
      if (
        error.code === "23505" ||
        error.message?.includes("duplicate") ||
        error.message?.includes("unique")
      ) {
        return { success: true, message: "Already on waitlist" };
      }
      throw error;
    }

    return { success: true, message: "You're on the waitlist" };
  } catch {
    return {
      success: false,
      message: "Something went wrong. Try again.",
    };
  }
}
