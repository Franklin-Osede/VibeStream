import { Song } from '../../../src/domain/entities/Song';

describe('Song Entity - TDD Approach', () => {
  describe('Creation', () => {
    it('should create a valid song with all required properties', () => {
      // ARRANGE
      const songData = {
        id: '1',
        title: 'Test Song',
        artist: 'Test Artist',
        artistId: 'artist1',
        duration: 180,
        genre: 'Pop',
        mood: 'Happy',
        imageUrl: 'https://example.com/image.jpg',
        audioUrl: 'https://example.com/audio.mp3',
      };

      // ACT
      const song = Song.create(
        songData.id,
        songData.title,
        songData.artist,
        songData.artistId,
        songData.duration,
        songData.genre,
        songData.mood,
        songData.imageUrl,
        songData.audioUrl
      );

      // ASSERT
      expect(song.isValid()).toBe(true);
      expect(song.metadata.title).toBe('Test Song');
      expect(song.metadata.artist).toBe('Test Artist');
      expect(song.metadata.duration).toBe(180);
      expect(song.stats.plays).toBe(0);
      expect(song.stats.likes).toBe(0);
      expect(song.stats.reposts).toBe(0);
    });

    it('should not create song with invalid data', () => {
      // ARRANGE & ACT
      const song = Song.create(
        '', // invalid id
        '', // invalid title
        '', // invalid artist
        'artist1',
        0, // invalid duration
        'Pop',
        'Happy',
        '', // invalid imageUrl
        '' // invalid audioUrl
      );

      // ASSERT
      expect(song.isValid()).toBe(false);
    });

    it('should format duration correctly', () => {
      // ARRANGE
      const song = Song.create(
        '1',
        'Test Song',
        'Test Artist',
        'artist1',
        125, // 2:05
        'Pop',
        'Happy',
        'https://example.com/image.jpg',
        'https://example.com/audio.mp3'
      );

      // ACT & ASSERT
      expect(song.metadata.getFormattedDuration()).toBe('2:5');
    });
  });

  describe('Business Logic - Play Functionality', () => {
    let song: Song;

    beforeEach(() => {
      song = Song.create(
        '1',
        'Test Song',
        'Test Artist',
        'artist1',
        180,
        'Pop',
        'Happy',
        'https://example.com/image.jpg',
        'https://example.com/audio.mp3'
      );
    });

    it('should increment plays when played', () => {
      // ARRANGE
      const initialPlays = song.stats.plays;

      // ACT
      song.play();

      // ASSERT
      expect(song.stats.plays).toBe(initialPlays + 1);
    });

    it('should increment plays multiple times', () => {
      // ARRANGE
      const expectedPlays = 5;

      // ACT
      for (let i = 0; i < expectedPlays; i++) {
        song.play();
      }

      // ASSERT
      expect(song.stats.plays).toBe(expectedPlays);
    });
  });

  describe('Business Logic - Like Functionality', () => {
    let song: Song;

    beforeEach(() => {
      song = Song.create(
        '1',
        'Test Song',
        'Test Artist',
        'artist1',
        180,
        'Pop',
        'Happy',
        'https://example.com/image.jpg',
        'https://example.com/audio.mp3'
      );
    });

    it('should toggle like state correctly', () => {
      // ARRANGE
      expect(song.state.isLiked).toBe(false);

      // ACT
      song.like();

      // ASSERT
      expect(song.state.isLiked).toBe(true);
      expect(song.stats.likes).toBe(1);

      // ACT
      song.unlike();

      // ASSERT
      expect(song.state.isLiked).toBe(false);
      expect(song.stats.likes).toBe(0);
    });

    it('should not allow negative likes', () => {
      // ARRANGE
      song.like(); // 1 like

      // ACT
      song.unlike(); // 0 likes
      song.unlike(); // Should still be 0

      // ASSERT
      expect(song.stats.likes).toBe(0);
    });

    it('should not increment likes if already liked', () => {
      // ARRANGE
      song.like(); // 1 like

      // ACT
      song.like(); // Should still be 1 like

      // ASSERT
      expect(song.stats.likes).toBe(1);
    });
  });

  describe('Business Logic - Repost Functionality', () => {
    let song: Song;

    beforeEach(() => {
      song = Song.create(
        '1',
        'Test Song',
        'Test Artist',
        'artist1',
        180,
        'Pop',
        'Happy',
        'https://example.com/image.jpg',
        'https://example.com/audio.mp3'
      );
    });

    it('should toggle repost state correctly', () => {
      // ARRANGE
      expect(song.state.isReposted).toBe(false);

      // ACT
      song.repost();

      // ASSERT
      expect(song.state.isReposted).toBe(true);
      expect(song.stats.reposts).toBe(1);

      // ACT
      song.unRepost();

      // ASSERT
      expect(song.state.isReposted).toBe(false);
      expect(song.stats.reposts).toBe(0);
    });
  });

  describe('Business Logic - Engagement Rate', () => {
    let song: Song;

    beforeEach(() => {
      song = Song.create(
        '1',
        'Test Song',
        'Test Artist',
        'artist1',
        180,
        'Pop',
        'Happy',
        'https://example.com/image.jpg',
        'https://example.com/audio.mp3'
      );
    });

    it('should calculate engagement rate correctly', () => {
      // ARRANGE
      song.play(); // 1 play
      song.like(); // 1 like
      song.repost(); // 1 repost

      // ACT
      const engagementRate = song.stats.getEngagementRate();

      // ASSERT
      expect(engagementRate).toBe(200); // (1+1)/1 * 100
    });

    it('should return 0 engagement rate for songs with no plays', () => {
      // ARRANGE
      song.like(); // 1 like
      song.repost(); // 1 repost

      // ACT
      const engagementRate = song.stats.getEngagementRate();

      // ASSERT
      expect(engagementRate).toBe(0); // No plays = 0 engagement
    });
  });

  describe('Blockchain Integration', () => {
    it('should handle songs without blockchain data', () => {
      // ARRANGE
      const song = Song.create(
        '1',
        'Test Song',
        'Test Artist',
        'artist1',
        180,
        'Pop',
        'Happy',
        'https://example.com/image.jpg',
        'https://example.com/audio.mp3'
      );

      // ACT & ASSERT
      expect(song.canBeTraded()).toBe(false);
      expect(song.hasFractionalOwnership()).toBe(false);
      expect(song.getRoyaltyAmount(100)).toBe(0);
    });

    it('should handle songs with blockchain data', () => {
      // ARRANGE
      const mockBlockchainData = {
        tokenId: 'token123',
        contractAddress: '0x123...',
        royalties: 0.1, // 10%
        fractionalOwnership: true,
        isValid: () => true,
        canBeTraded: () => true,
        getRoyaltyAmount: (price: number) => price * 0.1,
      };

      const song = Song.create(
        '1',
        'Test Song',
        'Test Artist',
        'artist1',
        180,
        'Pop',
        'Happy',
        'https://example.com/image.jpg',
        'https://example.com/audio.mp3',
        mockBlockchainData
      );

      // ACT & ASSERT
      expect(song.canBeTraded()).toBe(true);
      expect(song.hasFractionalOwnership()).toBe(true);
      expect(song.getRoyaltyAmount(100)).toBe(10);
    });
  });
}); 