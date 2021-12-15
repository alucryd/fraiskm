<script context="module" lang="ts">
  import { goto } from "$app/navigation";
  import { get } from "svelte/store";

  import Transition from "../components/Transition.svelte";
  import { me } from "../query.js";
  import { user } from "../store.js";

  export async function load({ page, fetch, session, stuff }) {
    await me();

    if (!get(user)) {
      goto("/signin", { replaceState: true });
    }

    return {};
  }
</script>

<Transition />
