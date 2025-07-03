import { Guild, Client } from 'discord.js';
import { createServer } from '../api.js';

export async function handleGuildJoin(guild: Guild, client: Client) {
	try {
		// Extract guild information
		const guildId = guild.id;
		const guildName = guild.name;

		console.log(`Bot joined guild: ${guildName} (${guildId})`);
		console.log(`Guild has ${guild.memberCount} members`);

		// Add the server to the database
		const server = await createServer(guildId, guildName);

		console.log(`✅ Successfully added server to database:`, {
			id: server.id,
			guild_id: server.guild_id,
			guild_name: server.guild_name,
			created_at: server.created_at,
		});
	} catch (error) {
		console.error(`❌ Error adding server ${guild.name} (${guild.id}) to database:`, error);
	}
}
