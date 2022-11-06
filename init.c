#include <stdio.h>
#include <stdbool.h>
#include <fcntl.h>
#include <sys/syscall.h>
#include <unistd.h>

#define MODULE_PATH "/rust_sample_module.ko"

int main()
{
    printf("init - entered main()\n");

    int iteration = 0;
    while (true)
    {
        printf("init - loading kernel module (iteration=%d)\n", iteration);

        int fd = open(MODULE_PATH, O_RDONLY);
        if (fd < 0)
        {
            printf("init - can not find %s\n", MODULE_PATH);
            return 255;
        }

        printf("init - module file fd=%d\n", fd);

        int ret = syscall(__NR_finit_module, fd, "my_bool=false", 0);

        if (ret != 0)
        {
            printf("init - module load failed: %d\n", ret);
        }
        else
        {
            printf("init - module loaded\n");
        }

        close(fd);

        if (syscall(__NR_delete_module, "rust_minimal", O_NONBLOCK) != 0)
        {
            printf("init - moduled delete failed\n");
        }
        else
        {
            printf("init - module deleted\n");
        }

        iteration++;
    }

    return 0;
}
