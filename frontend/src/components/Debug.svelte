<script module>
  import {
    get_all_save_keys,
    get_facts_ids_for,
    save_key_valid,
  } from "@/lib/data";
  import { OPENED_FACT, SAVE_FOUND } from "@/lib/stores";
  import {
    export_save_to_browser_url,
    get_save_from_browser_url,
  } from "@/lib/saves";
</script>

<script>
  let input_fact = $state("");
  let need_reload = $state(false);

  let is_fact_to_learn_valid = $derived(save_key_valid(input_fact));
  let is_fact_to_learn_already_learned = $derived(
    get_current_facts().has(input_fact),
  );
  let any_button_disabled = $derived(
    input_fact === "" || !is_fact_to_learn_valid,
  );
  let learn_button_disabled = $derived(
    any_button_disabled || is_fact_to_learn_already_learned,
  );
  let forget_button_disabled = $derived(
    any_button_disabled || !is_fact_to_learn_already_learned,
  );

  function get_current_facts() {
    return get_save_from_browser_url(get_all_save_keys());
  }

  function export_save(facts) {
    export_save_to_browser_url(get_all_save_keys(), facts);
    need_reload = true;
  }
  function learn_fact() {
    let current_facts = get_current_facts();
    current_facts.add(input_fact);
    export_save(current_facts);
  }
  function forget_fact() {
    let current_facts = get_current_facts();
    current_facts.delete(input_fact);
    export_save(current_facts);
  }
</script>

<div
  class="wrapper above-map-controls"
  class:hidden={!($OPENED_FACT || $SAVE_FOUND)}>
  {#if $OPENED_FACT}
    <div>
      Current entry:
      <br />
      <span class="mono">{$OPENED_FACT}</span>
    </div>
    <div class="padding-top">
      <div>Facts in entry:</div>
      <ul>
        {#each get_facts_ids_for($OPENED_FACT, true) as fact}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
          <li
            class="mono pointer"
            class:more-to-explore={fact.more_to_explore}
            onclick={() => (input_fact = fact.id)}>
            {fact.id}
          </li>
        {/each}
      </ul>
    </div>
  {/if}
  {#if $SAVE_FOUND}
    <div class="padding-top">
      <div>Learn/forget fact:</div>
      <div class="buttons padding-top">
        <input type="text" bind:value={input_fact} />
        <!-- <span></span> -->
        <button disabled={learn_button_disabled} onclick={learn_fact}
          >Learn</button>
        <button disabled={forget_button_disabled} onclick={forget_fact}
          >Forget</button>
        <button disabled={!need_reload} onclick={() => window.location.reload()}
          >Reload</button>
      </div>

      {#if input_fact !== ""}
        <div class="padding-top">
          {#if !is_fact_to_learn_valid}
            <i>Unknown fact id</i>
          {/if}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style lang="scss">
  $color: #3280ff;

  .wrapper {
    position: absolute;
    top: 0;
    right: 0;

    height: auto;
    padding: 1em 1em;

    background-color: var(--bg);
    border: 2px $color solid;
  }
  li {
    width: fit-content;
    padding: 1px 3px;

    &:hover {
      background-color: gray;
      border-radius: 3px;
    }
  }
  div:has(> .buttons) {
    max-width: 20em;
  }
  .buttons {
    display: flex;
    flex-wrap: wrap;

    & > * {
      margin: 2px;
    }

    & > input {
      width: 100%;
    }

    & > button {
      flex: 0 1 32%;
    }
  }
  .padding-top {
    padding-top: 5px;
  }
</style>
