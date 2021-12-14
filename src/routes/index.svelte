<script context="module" lang="ts">
  import { goto } from "$app/navigation";
  import { get } from "svelte/store";

  import { me } from "../query.js";
  import { user } from "../store.js";

  export async function load({ page, fetch, session, stuff }) {
    await me();

    if (!get(user)) {
      goto("/login", { replaceState: true });
    }

    return {};
  }
</script>
