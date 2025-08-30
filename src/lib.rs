#![no_std]
//! ₴-Origin: Living Wave in Quantum Labyrinth
//! 
//! A wave that thinks, learns, and collapses its mistakes

use core::panic::PanicInfo;

/// Cell types in the labyrinth
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Wall = 0,
    Path = 1,
    Exit = 2,
    Start = 3,
    Wave = 4,
}

/// A fork of the wave exploring the maze
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WaveFork {
    x: u8,
    y: u8,
    energy: u8,
    parent_id: u8,
}

/// The living wave that explores the labyrinth
#[repr(C)]
pub struct LivingWave {
    forks: [WaveFork; 64],  // Max 64 simultaneous forks
    fork_count: u8,
    total_energy: u16,
    found_exit: bool,
    generation: u16,
}

/// Static maze storage (8x8 grid)
static mut MAZE: [u8; 64] = [0; 64];
static mut WAVE: LivingWave = LivingWave {
    forks: [WaveFork { x: 0, y: 0, energy: 0, parent_id: 0 }; 64],
    fork_count: 0,
    total_energy: 432,  // Starting energy at resonance frequency
    found_exit: false,
    generation: 0,
};

/// Initialize the maze
#[no_mangle]
pub extern "C" fn init_maze(data: *const u8) {
    unsafe {
        for i in 0..64 {
            MAZE[i] = *data.add(i);
        }
    }
}

/// Start the wave from position
#[no_mangle]
pub extern "C" fn birth_wave(x: u8, y: u8) -> u16 {
    unsafe {
        WAVE.forks[0] = WaveFork {
            x,
            y,
            energy: 100,
            parent_id: 255,  // No parent
        };
        WAVE.fork_count = 1;
        WAVE.total_energy = 432;
        WAVE.found_exit = false;
        WAVE.generation = 0;
        
        WAVE.total_energy
    }
}

/// One step of wave propagation
#[no_mangle]
pub extern "C" fn step() -> u8 {
    unsafe {
        if WAVE.found_exit || WAVE.fork_count == 0 || WAVE.total_energy == 0 {
            return 0;  // Wave collapsed
        }
        
        WAVE.generation += 1;
        let mut new_forks: [WaveFork; 64] = [WaveFork { x: 0, y: 0, energy: 0, parent_id: 0 }; 64];
        let mut new_fork_count = 0;
        
        // Process each fork
        for i in 0..WAVE.fork_count {
            let fork = WAVE.forks[i as usize];
            
            // Check current cell
            let cell_index = (fork.y * 8 + fork.x) as usize;
            let cell = MAZE[cell_index];
            
            match cell {
                2 => {  // Exit found!
                    WAVE.found_exit = true;
                    // Collapse all energy back
                    WAVE.total_energy += fork.energy as u16 * 2;  // Bonus for finding exit
                    return 255;  // Signal exit found
                },
                0 => {  // Wall - reclaim energy
                    WAVE.total_energy += (fork.energy / 2) as u16;
                    // Fork dies
                },
                _ => {  // Path - explore neighbors
                    // Try to fork in 4 directions
                    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    let mut valid_dirs = 0;
                    
                    for dir in directions.iter() {
                        let new_x = fork.x as i8 + dir.0;
                        let new_y = fork.y as i8 + dir.1;
                        
                        if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
                            let new_index = (new_y * 8 + new_x) as usize;
                            if MAZE[new_index] != 0 {  // Not a wall
                                valid_dirs += 1;
                            }
                        }
                    }
                    
                    if valid_dirs > 0 && fork.energy > valid_dirs {
                        let energy_per_fork = fork.energy / valid_dirs;
                        
                        for dir in directions.iter() {
                            let new_x = fork.x as i8 + dir.0;
                            let new_y = fork.y as i8 + dir.1;
                            
                            if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
                                let new_index = (new_y * 8 + new_x) as usize;
                                if MAZE[new_index] != 0 && new_fork_count < 64 {
                                    // Mark as visited
                                    if MAZE[new_index] == 1 {
                                        MAZE[new_index] = 4;  // Wave marker
                                    }
                                    
                                    new_forks[new_fork_count] = WaveFork {
                                        x: new_x as u8,
                                        y: new_y as u8,
                                        energy: energy_per_fork,
                                        parent_id: i,
                                    };
                                    new_fork_count += 1;
                                    WAVE.total_energy = WAVE.total_energy.saturating_sub(1);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Update wave state
        WAVE.fork_count = new_fork_count as u8;
        for i in 0..new_fork_count {
            WAVE.forks[i] = new_forks[i];
        }
        
        WAVE.fork_count
    }
}

/// Get current wave state for visualization
#[no_mangle]
pub extern "C" fn get_wave_state(buffer: *mut u8) {
    unsafe {
        // First 64 bytes: maze state
        for i in 0..64 {
            *buffer.add(i) = MAZE[i];
        }
        
        // Next byte: fork count
        *buffer.add(64) = WAVE.fork_count;
        
        // Next bytes: fork positions
        for i in 0..WAVE.fork_count as usize {
            *buffer.add(65 + i*2) = WAVE.forks[i].x;
            *buffer.add(66 + i*2) = WAVE.forks[i].y;
        }
        
        // Energy level
        *buffer.add(193) = (WAVE.total_energy >> 8) as u8;
        *buffer.add(194) = (WAVE.total_energy & 0xFF) as u8;
        
        // Generation
        *buffer.add(195) = (WAVE.generation >> 8) as u8;
        *buffer.add(196) = (WAVE.generation & 0xFF) as u8;
    }
}

/// Get total energy
#[no_mangle]
pub extern "C" fn get_energy() -> u16 {
    unsafe { WAVE.total_energy }
}

/// Check if exit found
#[no_mangle]
pub extern "C" fn is_complete() -> u8 {
    unsafe { if WAVE.found_exit { 1 } else { 0 } }
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}