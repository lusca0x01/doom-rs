// Sound module implementation that bridges to Rust
#include "i_sound.h"
#include "w_wad.h"
#include "z_zone.h"
#include "doomtype.h"
#include <string.h>
#include <stdio.h>

// Rust functions
extern int rust_sound_init(void);
extern void rust_sound_shutdown(void);
extern int rust_sound_start(const unsigned char* data, unsigned int len, int channel, int vol, int sep);
extern void rust_sound_stop(int channel);
extern int rust_sound_is_playing(int channel);
extern void rust_sound_update_params(int channel, int vol, int sep);
extern void rust_sound_update(void);

extern int rust_music_init(void);
extern void rust_music_shutdown(void);
extern void rust_music_set_volume(int volume);
extern void rust_music_pause(void);
extern void rust_music_resume(void);
extern void rust_music_stop(void);
extern int rust_music_is_playing(void);

static boolean use_prefix = false;

static boolean DG_Sound_Init(boolean _use_sfx_prefix)
{
    use_prefix = _use_sfx_prefix;
    return rust_sound_init() != 0;
}

static void DG_Sound_Shutdown(void)
{
    rust_sound_shutdown();
}

static int DG_Sound_GetSfxLumpNum(sfxinfo_t *sfxinfo)
{
    char namebuf[16];
    
    if (use_prefix)
    {
        snprintf(namebuf, sizeof(namebuf), "ds%s", sfxinfo->name);
    }
    else
    {
        snprintf(namebuf, sizeof(namebuf), "%s", sfxinfo->name);
    }
    
    return W_CheckNumForName(namebuf);
}

static void DG_Sound_Update(void)
{
    rust_sound_update();
}

static void DG_Sound_UpdateParams(int channel, int vol, int sep)
{
    rust_sound_update_params(channel, vol, sep);
}

static int DG_Sound_StartSound(sfxinfo_t *sfxinfo, int channel, int vol, int sep)
{
    int lumpnum;
    unsigned char *data;
    int lumplen;
    
    lumpnum = sfxinfo->lumpnum;
    
    if (lumpnum < 0)
    {
        return -1;
    }
    
    data = W_CacheLumpNum(lumpnum, PU_STATIC);
    lumplen = W_LumpLength(lumpnum);
    
    int result = rust_sound_start(data, lumplen, channel, vol, sep);
    
    W_ReleaseLumpNum(lumpnum);
    
    return result;
}

static void DG_Sound_StopSound(int channel)
{
    rust_sound_stop(channel);
}

static boolean DG_Sound_IsPlaying(int channel)
{
    return rust_sound_is_playing(channel) != 0;
}

static void DG_Sound_CacheSounds(sfxinfo_t *sounds, int num_sounds)
{
    // Pre-cache sounds - we handle this on-demand
    (void)sounds;
    (void)num_sounds;
}

static snddevice_t sound_devices[] = 
{
    SNDDEVICE_SB,
    SNDDEVICE_PAS,
    SNDDEVICE_GUS,
    SNDDEVICE_WAVEBLASTER,
    SNDDEVICE_SOUNDCANVAS,
    SNDDEVICE_AWE32,
};

sound_module_t DG_sound_module = 
{
    sound_devices,
    sizeof(sound_devices) / sizeof(*sound_devices),
    DG_Sound_Init,
    DG_Sound_Shutdown,
    DG_Sound_GetSfxLumpNum,
    DG_Sound_Update,
    DG_Sound_UpdateParams,
    DG_Sound_StartSound,
    DG_Sound_StopSound,
    DG_Sound_IsPlaying,
    DG_Sound_CacheSounds,
};

// Music module - basic stub implementation

static boolean DG_Music_Init(void)
{
    return rust_music_init() != 0;
}

static void DG_Music_Shutdown(void)
{
    rust_music_shutdown();
}

static void DG_Music_SetVolume(int volume)
{
    rust_music_set_volume(volume);
}

static void DG_Music_Pause(void)
{
    rust_music_pause();
}

static void DG_Music_Resume(void)
{
    rust_music_resume();
}

static void* DG_Music_RegisterSong(void *data, int len)
{
    (void)data;
    (void)len;
    return (void*)1;
}

static void DG_Music_UnRegisterSong(void *handle)
{
    (void)handle;
}

static void DG_Music_PlaySong(void *handle, boolean looping)
{
    (void)handle;
}

static void DG_Music_StopSong(void)
{
    rust_music_stop();
}

static boolean DG_Music_IsPlaying(void)
{
    return rust_music_is_playing() != 0;
}

static void DG_Music_Poll(void)
{
    // Nothing to do
}

static snddevice_t music_devices[] =
{
    SNDDEVICE_SB,
    SNDDEVICE_PAS,
    SNDDEVICE_GUS,
    SNDDEVICE_WAVEBLASTER,
    SNDDEVICE_SOUNDCANVAS,
    SNDDEVICE_GENMIDI,
    SNDDEVICE_AWE32,
};

music_module_t DG_music_module =
{
    music_devices,
    sizeof(music_devices) / sizeof(*music_devices),
    DG_Music_Init,
    DG_Music_Shutdown,
    DG_Music_SetVolume,
    DG_Music_Pause,
    DG_Music_Resume,
    DG_Music_RegisterSong,
    DG_Music_UnRegisterSong,
    DG_Music_PlaySong,
    DG_Music_StopSong,
    DG_Music_IsPlaying,
    DG_Music_Poll,
};

void use_libsamplerate(void) {}
void libsamplerate_scale(void) {}