import {
  Box,
  Text,
  IconButton,
  Badge,
  Divider,
  Button,
} from "@chakra-ui/react";
import { ExamEnvironmentConfig } from "@prisma/client";
import { DiffField } from "./diff-field";

interface ConfigViewProps {
  config: ExamEnvironmentConfig;
  setConfig: (partialConfig: Partial<ExamEnvironmentConfig>) => void;
}

export function ConfigView({ config, setConfig }: ConfigViewProps) {
  return (
    <>
      <DiffField
        currentValue={config.tags}
        getDeployedValue={(exam) => exam.config.tags}
      >
        <Box>
          {config.tags?.map((tagConfig, index) => (
            <Box key={index} className="tag-config-container" mb={2}>
              <Text fontWeight="bold" color="gray.100">
                Config {index + 1} ({tagConfig.numberOfQuestions} Questions)
              </Text>
              {tagConfig.group.map((tag, inner) => (
                <Badge
                  key={inner}
                  colorScheme="teal"
                  variant="subtle"
                  mr={1}
                  mb={1}
                >
                  {tag}
                </Badge>
              ))}
              <IconButton
                aria-label="Remove"
                icon={<span>✕</span>}
                size="xs"
                ml={2}
                colorScheme="red"
                variant="ghost"
                onClick={() => {
                  setConfig({
                    tags: config.tags.filter((_, i) => i !== index),
                  });
                }}
              />
            </Box>
          ))}
        </Box>
      </DiffField>
      <Divider my={4} borderColor="gray.600" />
      <DiffField
        currentValue={config.questionSets}
        getDeployedValue={(exam) => exam.config.questionSets}
      >
        <Box>
          {config.questionSets.map((qt, index) => (
            <Box key={index} className="tag-config-container" mb={2}>
              <Text fontWeight="bold" color="gray.100">
                {qt.type} Questions
              </Text>
              <Text color="gray.300" fontSize="sm">
                Number of Type: {qt.numberOfSet}
              </Text>
              <Text color="gray.300" fontSize="sm">
                Number of Questions: {qt.numberOfQuestions}
              </Text>
              <Text color="gray.300" fontSize="sm">
                Number of Correct Answers: {qt.numberOfCorrectAnswers}
              </Text>
              <Text color="gray.300" fontSize="sm">
                Number of Incorrect Answers: {qt.numberOfIncorrectAnswers}
              </Text>
              <Button
                size="xs"
                colorScheme="red"
                mt={1}
                onClick={() =>
                  setConfig({
                    questionSets: config.questionSets.filter(
                      (_, i) => i !== index
                    ),
                  })
                }
              >
                Remove
              </Button>
            </Box>
          ))}
        </Box>
      </DiffField>
    </>
  );
}
