//! Ype ÆNDROID

use std::process::Command;

/// Проверяет, установлены ли необходимые пакеты.
fn check_dependencies() {
  let status = Command::new("/usr/bin/pacman")
                 .arg("-Qi")
                 .arg("anbox-git")
                 .status()
                 .expect("Менеджер пакетов pacman не найден. Убедитесь, что программа запущена на Arch-совместимом дистрибутиве.");
   status.exit_ok().expect_err("Пакет anbox-git не установлен.");
   let status = Command::new("/usr/bin/pacman")
                 .arg("-Qi")
                 .arg("anbox-support")
                 .status()
                 .expect("Менеджер пакетов pacman не найден. Убедитесь, что программа запущена на Arch-совместимом дистрибутиве.");
   status.exit_ok().expect_err("Пакет anbox-support не установлен.");
   let status = Command::new("/usr/bin/pacman")
                 .arg("-Qs")
                 .arg("anbox-image")
                 .status()
                 .expect("Менеджер пакетов pacman не найден. Убедитесь, что программа запущена на Arch-совместимом дистрибутиве.");
   status.exit_ok().expect_err("Ни один из образов Android-системы для Anbox не установлен. Запустите '$ pacman -Ss anbox-image' для просмотра вариантов.");
}

/// Точка входа.
fn main() {
  check_dependencies();
}
