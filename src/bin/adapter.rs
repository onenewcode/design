
use std::error::Error;
// 播放器接口
trait  MediaPlayer{
    fn paly(&mut self,audio_type:String, file_name:String);
}
// 适配器接口
trait AdvancedMediaPlayer {
    fn play_vlc(&self ,file_name:String);
    fn play_mp4(&self,file_name:String );
}
// vlc播器放实体类
struct  VlcPlayer {}
impl AdvancedMediaPlayer for VlcPlayer {
    fn play_vlc(&self ,file_name:String) {
        println!("Playing vlc file. Name: {}",file_name)
    }

    fn play_mp4(&self,file_name:String ) {
        todo!()
    }
}
// MP4播放器实体类
struct  Mp4Player {}
impl AdvancedMediaPlayer for Mp4Player {
    fn play_mp4(&self ,file_name:String) {
        println!("Playing mp4 file. Name: {}",file_name)
    }

    fn play_vlc(&self,file_name:String ) {
        todo!()
    }
}
// 播放器适配器实体类
struct MediaAdapter{
    advanced_music_player:Box<dyn AdvancedMediaPlayer>
}
impl MediaPlayer for MediaAdapter {
    fn paly(&mut self,audio_type:String, file_name:String) {
        match &audio_type as &str{
            "vlc"=> self.advanced_music_player.play_vlc(file_name),
            "mp4"=>self.advanced_music_player.play_mp4(file_name),
            _ =>println!("不支持此格式文件")
        }
    }
}
impl MediaAdapter {
    fn new (audio_type:String )->Result<MediaAdapter,Box<dyn Error>>{
        //进行字符产匹配
        match &audio_type as &str{
            "vlc"=> Ok(MediaAdapter{advanced_music_player:Box::new(VlcPlayer{})}),
            "mp4"=>Ok(MediaAdapter{advanced_music_player:Box::new(Mp4Player{})}),
            _ =>Err(panic!("输入运行格式错误"))
        }
    }
}
struct AudioPlayer{
    media_adapter:MediaAdapter 
}
impl AudioPlayer {
    fn new()->AudioPlayer{
        AudioPlayer{media_adapter:MediaAdapter { advanced_music_player:Box::new(Mp4Player{})}}
    }
}
impl  MediaPlayer for AudioPlayer {
    fn paly(&mut self,audio_type:String, file_name:String) {
        match &audio_type as &str{
            "vlc"|"mp4"=>{
                self.media_adapter=MediaAdapter::new(audio_type.clone()).unwrap();
                self.media_adapter.paly(audio_type, file_name);
            },
            _ =>println!("不支持此格式文件")
        }
    }
}
    
fn main() {
    let mut audioPlayer =AudioPlayer::new();
    // 进行不同文件的播放测试
    audioPlayer.paly(String::from("mp3"), String::from("beyond the horizon.mp3"));
    audioPlayer.paly(String::from("mp4"), String::from("alone.mp4"));
    audioPlayer.paly(String::from("mp4"), String::from("far far away.vlc"));
}