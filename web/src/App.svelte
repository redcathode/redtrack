<script>
    import { onMount } from 'svelte';
    let posts = [];
    let newPost = {
        notes: '',
        overall: 0,
        psychomotor: 0,
        energy: 0,
        mood: 0,
        thoughts_slowed_racing: 0,
        concentration_difficulty: 0
    };

	let disabledFields = {
        notes: false,
        overall: false,
        psychomotor: false,
        energy: false,
        mood: false,
        thoughts_slowed_racing: false,
        concentration_difficulty: false
    };

    onMount(async () => {
        const response = await fetch('http://localhost:3000/posts');
        posts = await response.json();
    });

    async function submitPost() {
        const response = await fetch('http://localhost:3000/posts/submit', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(newPost)
        });
        const newPostResponse = await response.json();
        posts = [...posts, newPostResponse];
        newPost = {
            notes: '',
            overall: 0,
            psychomotor: 0,
            energy: 0,
            mood: 0,
            thoughts_slowed_racing: 0,
            concentration_difficulty: 0
        };
    }
</script>

<main>
    <h1>Posts</h1>
    {#each posts as post (post.id)}
        <article>
            <h2>Notes: {post.notes}</h2>
            <p>Overall: {post.overall}</p>
            <p>Psychomotor: {post.psychomotor}</p>
            <p>Energy: {post.energy}</p>
            <p>Mood: {post.mood}</p>
            <p>Thoughts Slowed/Racing: {post.thoughts_slowed_racing}</p>
            <p>Concentration Difficulty: {post.concentration_difficulty}</p>
        </article>
    {/each}
    <form on:submit|preventDefault={submitPost}>
		<label>
			Notes:
			<input type="text" bind:value={newPost.notes} disabled={disabledFields.notes}>
			<input type="checkbox" bind:checked={disabledFields.notes}> Disable
		</label>
        <label>
            Overall:
            <input type="range" min="-5" max="5" bind:value={newPost.overall} disabled={disabledFields.overall}>
            <input type="number" min="-5" max="5" disabled={disabledFields.overall}>
            <input type="checkbox" bind:checked={disabledFields.overall}> Disable
        </label>
		<!-- Add other fields as needed -->
		<button type="submit">Submit Post</button>
	</form>
</main>

<style>
    main {
        padding: 1em;
        max-width: 600px;
        margin: 0 auto;
    }
    article {
        border-bottom: 1px solid #ccc;
        padding-bottom: 1em;
        margin-bottom: 1em;
    }
    form {
        display: flex;
        flex-direction: column;
        gap: 1em;
    }
</style>