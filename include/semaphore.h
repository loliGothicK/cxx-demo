#pragma once
#include <memory>
#include <semaphore>

namespace mitama
{
    class Semaphore
    {
    public:
        Semaphore(uint64_t count);

        auto acquire() const -> void;
        auto release() const -> void;
        auto try_acquire() const -> bool;

    private:
        mutable std::counting_semaphore<std::counting_semaphore<>::max()> semaphore;
    };

    std::unique_ptr<Semaphore> new_counting_semaphore(uint64_t count);
}
