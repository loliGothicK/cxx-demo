#include "cxx-demo/include/semaphore.h"

namespace mitama
{
    Semaphore::Semaphore(uint64_t count) : semaphore(count) {}

    auto Semaphore::acquire() const -> void
    {
        this->semaphore.acquire();
    }

    auto Semaphore::release() const -> void
    {
        this->semaphore.release();
    }

    auto Semaphore::try_acquire() const -> bool
    {
        return this->semaphore.try_acquire();
    }

    std::unique_ptr<Semaphore> new_counting_semaphore(uint64_t count)
    {
        return std::make_unique<Semaphore>(count);
    }
}
