mod util;

use intcode::*;
use util::*;

use std::{
  collections::{VecDeque, HashMap},
  io,
  thread::{sleep},
  time::{Duration, Instant}
};

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::{
  backend::TermionBackend,
  buffer::Buffer,
  layout::{Alignment, Layout, Rect, Constraint, Direction},
  style::{Color, Modifier, Style},
  widgets::{
    canvas::{
      Canvas,
      Line, 
      Map, 
      MapResolution, 
      Rectangle
    },
    Block, 
    Borders, 
    Paragraph,
    Text,
    Widget
  },
  Terminal
};


#[derive(Copy, Clone, PartialEq)]
enum TileId {
  Empty,
  Wall,
  Block,
  HorizontalPaddle,
  Ball
}

struct ArcadeScreen<'a> {
  offset: (isize, isize),
  screen: &'a HashMap<(isize, isize), TileId>
}

impl <'a> ArcadeScreen<'a> {
  fn new(screen: &'a HashMap<(isize, isize), TileId>, offset: (isize, isize)) -> Self {
    ArcadeScreen { screen, offset }
  }
}

impl Widget for ArcadeScreen<'_> {
  fn draw(&mut self, area: Rect, buf: &mut Buffer) {
    for x in area.left()..area.right() {
      for y in area.top()..area.bottom() {
        let cell = buf.get_mut(x, y);
        let tile = self.screen.get(&(x as isize + self.offset.0, y as isize + self.offset.1)).unwrap_or(&TileId::Empty);

        match tile {
          TileId::Empty => cell.set_char(' '),
          TileId::Ball => cell.set_char('.'),
          TileId::Block => cell.set_char('X'),
          TileId::HorizontalPaddle => cell.set_char('='),
          TileId::Wall => cell.set_char('█')
        };
      }
    }
  }
}


fn repaint_screen<T: tui::backend::Backend>(terminal: &mut Terminal<T>, screen: &HashMap<(isize, isize), TileId>, score: isize) {
  terminal.draw(|mut f| {
    let chunks = Layout::default()
          .direction(Direction::Horizontal)
          .constraints([Constraint::Min(30), Constraint::Length(10)].as_ref())
          .split(f.size());

    let screen_rect = chunks[0];
    let mut border = Block::default()
        .title(" Intcode Arcade ")
        .borders(Borders::ALL);
    border.render(&mut f, screen_rect);
    
    ArcadeScreen::new(screen, (-1, -1))
      .render(&mut f, border.inner(screen_rect));


    let side_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
        .split(chunks[1]);

    let score_rect = side_chunks[0];
    
    let text = [
      Text::raw(format!("{}", score))
    ];
    Paragraph::new(text.iter())
        .block(Block::default()
          .title(" Score ")
          .borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(true)
        .render(&mut f, score_rect);
  });
}

pub fn main() -> Result<(), failure::Error> {
  println!("Starting arcade cabinet");

  let input = include_str!("../../input/2019/day13.txt");
  let mut program = &intcode_parser(input);
  let mut machine = Machine::new(program);

  // Terminal initialization
  {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let events = Events::new();

    let mut output_buffer = VecDeque::new();
    let mut screen = HashMap::new();

    let mut automatic = false;
    let mut fast = false;
    let mut ball_position = (0, 0);
    let mut paddle_position = (0, 0);
    let mut score = 0;
    let mut game_input = 0;

    machine.write(2, &Parameter::Position(0));

    let mut last_frame_time = Instant::now();

    'main: loop {
      match machine.step() {
        Some(Action::RequiresInput) => {
          if automatic {          
            if ball_position.0 < paddle_position.0 {
              machine.push_input(-1);
            } else if ball_position.0 > paddle_position.0 {
              machine.push_input(1);
            } else {
              machine.push_input(0);
            }
          } else {
            machine.push_input(game_input);
          }

          let frame_time = last_frame_time.elapsed().as_millis();
          let min_frame_time = 500;
          if !fast && frame_time < min_frame_time {
            sleep(Duration::from_millis((min_frame_time - frame_time) as u64));
          }
          last_frame_time = Instant::now();


          // Handle input
          'events: loop {
            match events.poll() {
              Ok(Event::Input(input)) => match input {
                  Key::Esc => { break 'main; },
                  Key::Char('a') => { game_input = -1; },
                  Key::Char('o') => { game_input = 0; },
                  Key::Char('e') => { game_input = 1; },
                  Key::Char('l') => { automatic = !automatic; },
                  Key::Char('f') => { fast = !fast; },
                  _ => {}
              },
              Ok(_) => {},

              Err(std::sync::mpsc::TryRecvError::Empty) => { break 'events; },
              Err(_) => { break 'main; }
            }
          }
        },
        Some(Action::Halt) => break 'main,
        Some(Action::Output(value)) => {
          output_buffer.push_back(value);
          
          if output_buffer.len() >= 3 {
            let x = output_buffer.pop_front().unwrap();
            let y = output_buffer.pop_front().unwrap();
            let value = output_buffer.pop_front().unwrap();

            if x == -1 && y == 0 {
              score = value;
            } else {
              let tile_id = match value {
                0 => TileId::Empty,
                1 => TileId::Wall,
                2 => TileId::Block,
                3 => TileId::HorizontalPaddle,
                4 => TileId::Ball,
                _ => unimplemented!()
              };

              screen.insert((x, y), tile_id);

              if tile_id == TileId::Ball {
                ball_position = (x, y);
              } else if tile_id == TileId::HorizontalPaddle {
                paddle_position = (x, y);
              }
            }

            repaint_screen(&mut terminal, &screen, score);
          }
        },
        None => {}
      }
    }

    std::thread::sleep_ms(10000);
  }
  println!("Arcade stopped");
  Ok(())
}